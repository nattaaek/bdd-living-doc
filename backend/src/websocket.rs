use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_ws::Message;
use bytestring::ByteString;
use futures_util::StreamExt;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

type Tx = UnboundedSender<ByteString>;

#[derive(Clone)]
pub struct WsServer {
    pub clients: Arc<Mutex<Vec<Tx>>>,
}

impl WsServer {
    pub fn new() -> Self {
        WsServer {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn broadcast_update(&self) {
        let clients = self.clients.lock().unwrap();
        for tx in clients.iter() {
            // Send a ByteString directly
            let _ = tx.send("update".into());
        }
    }
}

#[get("/ws/")]
pub async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    ws_server: web::Data<WsServer>,
) -> Result<HttpResponse, actix_web::Error> {
    let ws_server = ws_server.get_ref().clone();

    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    // Create a channel to send messages to this client
    let (tx, mut rx) = unbounded_channel::<ByteString>();

    // Add the sender to the list of clients
    {
        let mut clients = ws_server.clients.lock().unwrap();
        clients.push(tx);
    }

    // Spawn a task to forward messages from the server to the client
    actix_web::rt::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let _ = session.text(msg).await;
        }
    });

    // Spawn a task to handle incoming messages from the client
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(_text) => {
                    // Handle incoming text messages if needed
                }
                Message::Close(_) => {
                    // Remove the sender from the clients list
                    let mut clients = ws_server.clients.lock().unwrap();
                    clients.retain(|client_tx| !client_tx.is_closed());
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(response)
}
