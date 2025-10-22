use moka::future::Cache;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::sync::{LazyLock, Mutex};
use tokio::sync::OnceCell;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)] // 枚举不带 tag 即 {"Int":42} => 42
pub enum Value {
    String(String),
    Integer(i64),
    Boolean(bool),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

type ConfigCallBack = Box<dyn Fn(&str, &Value) + Send + Sync>;
type CallbackList = Mutex<Vec<ConfigCallBack>>;

static CONFIG_PATH: &str = "config.json";
static ON_CHANGE_CALLBACK_ARR: LazyLock<CallbackList> = LazyLock::new(|| Mutex::new(vec![]));
static CACHE: OnceCell<Cache<String, Value>> = OnceCell::const_new();

pub fn register_on_change_callback<F>(callback: F)
where
    F: Fn(&str, &Value) + Send + Sync + 'static,
{
    ON_CHANGE_CALLBACK_ARR
        .lock()
        .unwrap()
        .push(Box::new(callback));
}

fn read_config() -> Result<HashMap<String, Value>, Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(CONFIG_PATH)?;
    let reader = BufReader::new(file);
    let config: HashMap<String, Value> = serde_yaml::from_reader(reader)?;
    Ok(config)
}

pub async fn init_config() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match read_config() {
        Ok(config) => {
            for (key, value) in config {
                get_cache().await.insert(key, value).await;
            }
        }
        Err(e) => eprintln!("Failed to load config: {}", e),
    }
    Ok(())
}

async fn write_config() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cache: HashMap<String, Value> = get_cache()
        .await
        .iter()
        .map(|(k, v)| (k.as_ref().clone(), v.clone()))
        .collect();
    let mut file = File::create(CONFIG_PATH)?;
    let mut writer = BufWriter::new(&mut file);
    serde_yaml::to_writer(&mut writer, &cache)?;
    Ok(())
}

fn zero_value_for_key(key: &str) -> Value {
    if key.ends_with("Switch") || key.ends_with("Enabled") {
        Value::Boolean(false)
    } else if key.ends_with("Slice") || key.ends_with("List") || key.ends_with("Array") {
        Value::List(vec![])
    } else {
        Value::String("".to_string())
    }
}

pub fn extract_bool(value: &Value) -> Option<bool> {
    match value {
        Value::Boolean(b) => Some(*b),
        Value::Map(m) => {
            if let Some(Value::Boolean(b)) = m.get("value") {
                Some(*b)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub async fn get_cache() -> &'static Cache<String, Value> {
    CACHE
        .get_or_init(|| async {
            let cache = Cache::builder().build();
            match read_config() {
                Ok(config) => {
                    for (k, v) in config {
                        cache.insert(k.clone(), v.clone()).await; // 在 async 块中可以自由 .await
                    }
                }
                Err(_) => {}
            }
            cache
        })
        .await
}

pub async fn get_config(key: &str) -> Result<Value, String> {
    match get_cache().await.get(key).await {
        Some(v) => {
            log::debug!("Config get: {} = {:?}", key, v);
            Ok(v)
        }
        None => {
            let zero_val = zero_value_for_key(key);
            log::debug!("Config get (default): {} = {:?}", key, zero_val);
            Ok(zero_val)
        }
    }
}

pub async fn put_config(key: String, value: Value) -> Result<(), String> {
    get_cache().await.insert(key.clone(), value.clone()).await;
    for callback in ON_CHANGE_CALLBACK_ARR.lock().unwrap().iter() {
        callback(&key, &value);
    }
    write_config().await.map_err(|e| e.to_string())
}
