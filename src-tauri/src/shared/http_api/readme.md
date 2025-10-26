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
