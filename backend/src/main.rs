mod file_watcher;
mod parser;
mod websocket;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;

use websocket::WsServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let features = Arc::new(Mutex::new(parser::parse_feature_files()));
    let features_clone = Arc::clone(&features);

    // Create an instance of WsServer
    let ws_server = WsServer::new();
    let ws_server_clone = ws_server.clone();

    let (tx, rx) = channel();

    // Start file watcher in a separate thread
    std::thread::spawn(move || {
        file_watcher::watch_feature_files(tx).unwrap();
    });

    // Start a thread to handle file change events
    let features_clone_inner = Arc::clone(&features_clone);
    std::thread::spawn(move || {
        for _event in rx {
            let new_features = parser::parse_feature_files();
            let mut features_lock = features_clone_inner.lock().unwrap();
            *features_lock = new_features;
            // Notify WebSocket clients about the update
            ws_server_clone.broadcast_update(); // Correct method call
        }
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(Arc::clone(&features_clone)))
            .app_data(web::Data::new(ws_server.clone()))
            .route("/features", web::get().to(get_features))
            .service(websocket::websocket_handler)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_features(data: web::Data<Arc<Mutex<Vec<gherkin::Feature>>>>) -> impl Responder {
    let features = data.lock().unwrap();
    HttpResponse::Ok().json(&*features)
}
