use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct Demo {
    pub name: String,
    pub age: u32,
}

#[derive(Default, Debug)]
pub struct DemoManager {
    pub state: RwLock<Demo>,
}

fn main() {
    let demo = DemoManager::default();
    println!("{:?}", demo);
}
