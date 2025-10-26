由于 Rust 没有 MobX 这样的响应式库，所以需要自己实现响应式设计，在这里我们可以尝试将 属性字段都隐藏掉，同时使用 get set 方法来获取字段。并使用 channels 来处理后端到前端的响应

其响应式设计主要是为了在 数据变化时，自动触发 UI 的更新等等事件，

这是一个后端向前端数据更新的过程，在这里 tauri 可以使用 event system，channels， evaluating js 三种方法来实现：https://tauri.app/develop/calling-frontend/#channels

这里推荐使用 channels 方法，其性能最好。

后续 lcu_state 结构体中需要使用 `Arc<Rwlock>` 来封装或者 `Arc<Mutex>` 来封装

```rust
后续可能实现
// 事件类型
#[derive(Clone, Serialize)]
pub enum GameDataEvent {
    SummonerSpellsUpdated { data: HashMap<i32, SummonerSpell> },
    ItemsUpdated { data: HashMap<i32, Item> },
    QueuesUpdated { data: HashMap<i32, Queue> },
    PerksUpdated { data: HashMap<i32, Perk> },
    PerkStylesUpdated { data: PerkStylesData },
    AugmentsUpdated { data: HashMap<i32, Augment> },
    ChampionsUpdated { data: HashMap<i32, ChampionSimple> },
}


// 辅助方法：发射事件
fn emit_event(&self, app_handle: &tauri::AppHandle, event: GameDataEvent) {
    let _ = app_handle.emit_all("game-data-updated", event);
}
```

Websocket 样式：

```rust
/// server.rs
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message;
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");

    println!("WebSocket server listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    println!("New WebSocket connection");

    let (mut write, mut read) = ws_stream.split();

    // 给客户端发送一个事件消息
    let event_msg = "[5,\"OnJsonApiEvent\"]";
    if let Err(e) = write.send(Message::Text(event_msg.into())).await {
        eprintln!("Send error: {}", e);
        return;
    }

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(txt)) => {
                println!("Received from client: {}", txt);
            }
            Ok(Message::Close(_)) => {
                println!("Client disconnected");
                break;
            }
            _ => {}
        }
    }
}

```

```rust
/// client.rs
use std::error::Error;
use std::thread;
use tungstenite::{client::IntoClientRequest, connect, Message};
use url::Url;

fn main() -> Result<(), Box<dyn Error>> {
    let url = Url::parse("ws://127.0.0.1:9001")?;
    let (mut ws_stream, _response) = connect(url.to_string().into_client_request()?)?;

    println!("✅ WebSocket Connected");

    // 订阅消息
    let msg = "[5, \"OnJsonApiEvent\"]";
    ws_stream.write(Message::Text(msg.into()))?;

    // 接收消息
    thread::spawn(move || loop {
        match ws_stream.read() {
            Ok(Message::Text(txt)) => println!("📌 Event: {}", txt),
            Ok(Message::Close(_)) => {
                println!("🔌 WebSocket closed");
                break;
            }
            Err(err) => {
                println!("❌ WebSocket error: {:?}", err);
                break;
            }
            _ => {}
        }
    });

    loop {
        thread::park();
    }
}
```

### OJBK Websocket 搞定了

```rust
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use futures::SinkExt;
use futures_util::{future, pin_mut, StreamExt};
use native_tls::TlsConnector;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::protocol::Message};
use tungstenite::client::IntoClientRequest;

#[tokio::main]
async fn main() {
    let auth_token = "TaTfqECOgbX9P_GOmPjK4A";
    let url = "wss://127.0.0.1:56389/";

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    // handshake with the server
    let connector = tokio_tungstenite::Connector::NativeTls(TlsConnector::from(
        native_tls::TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .build()
            .unwrap(),
    ));

    let mut request = url.into_client_request().unwrap();
    let encode = STANDARD.encode(format!("riot:{}", auth_token));
    println!("{}", format!("Basic {}", encode));

    request.headers_mut().insert(
        "Authorization",
        format!("Basic {}", encode).parse().unwrap(),
    );

    let (ws_stream, _) = connect_async_tls_with_config(request, None, false, Some(connector))
        .await
        .expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (mut write, read) = ws_stream.split();

    write
        .send(Message::Text("[5,\"OnJsonApiEvent\"]".into()))
        .await
        .unwrap();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async move {
            let a = message.as_ref().unwrap().to_text().unwrap();
            let json_data: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(a);
            if let Ok(json_data) = json_data {
                tokio::io::stdout()
                    .write_all(serde_json::to_string_pretty(&json_data).unwrap().as_bytes())
                    .await
                    .unwrap();
            }
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}
```

#### 1. 订阅事件

发送 ： [5, "OnJsonApiEvent"]

#### 2. 退订事件

发送 ： [6, "OnJsonApiEvent"]

#### 3. 事件格式

得到 ：[8,”OnJsonApiEvent”,{“data”:[],”eventType”:”Update”,”uri”:”/lol-ranked/v1/notifications”}]
