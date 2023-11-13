use http::Uri;
use futures_util::{future, pin_mut, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{Sender, Receiver, self};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Serialize, Deserialize)]
pub struct WsMessage {
    pub chat_id: String,
    pub sender_id: String,
    pub content: Vec<u8>,
    pub timestamp: u64,
}

pub struct ThreadComm<T> {
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
}

fn generate_key() -> String {
    let k: [u8; 16] = rand::random();
    data_encoding::BASE64.encode(&k)
}

pub struct Ws {
    url: Uri,
    token: String,
}

impl Ws {
    pub fn new(url: &str, token: &str) -> Self {
        Ws {
            url: url.parse()
                .expect("Url is hardcoded"),
            token: String::from(token),
        }
    }

    pub async fn run(&self) -> ThreadComm<WsMessage> {
        let (tx, rx) = mpsc::channel::<WsMessage>(1000);
        let (tx2, rx2) = mpsc::channel::<WsMessage>(1000);

        let req = self.make_request()
            .expect("Url is hardcoded");
        tokio::spawn(async {Self::run_impl(req, tx2, rx)}.await);

        ThreadComm {
            sender: tx,
            receiver: rx2,
        }
    }

    async fn run_impl(req: http::Request<()>, sender: Sender<WsMessage>, mut receiver: Receiver<WsMessage>) {
        let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
        tokio::spawn(async move {
            loop {
                let n = match receiver.recv().await {
                    Some(s) => bincode::serialize(&s).unwrap(),
                    None => break,
                };
                stdin_tx.unbounded_send(Message::binary(n)).unwrap();
            }
        });

        let (ws_stream, _) = connect_async(req).await.expect("Failed to connect");

        let (write, read) = ws_stream.split();

        let stdin_to_ws = stdin_rx.map(Ok).forward(write);
        let ws_to_stdout: _ = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                let data: WsMessage = bincode::deserialize(&data).unwrap();
                sender.send(data).await.expect("Sender disconnected");
            })
        };

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
    }

    fn make_request(&self) -> Option<http::Request<()>> {
        let authority = self.url.authority().unwrap().as_str();
        let host = authority
            .find('@')
            .map(|idx| authority.split_at(idx + 1).1)
            .unwrap_or_else(|| authority);

        if host.is_empty() {
            return None;
        }

        let req = http::Request::builder()
            .method("GET")
            .header("Host", host)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", generate_key())
            .header("Authorization", format!("Bearer {}", self.token))
            .uri(&self.url)
            .body(())
            .unwrap();
        Some(req)
    }
}
