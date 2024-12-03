use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    from: String,
    to: String,
    message: String,
}

#[tokio::main]
async fn main() {
    
    tokio::spawn(async {
        serve_html().await;
    });
    
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    let clients = Arc::new(Mutex::new(HashMap::new()));

    println!("WebSocket server running at ws://127.0.0.1:8081");

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream)
            .await
            .expect("Error during WebSocket handshake");

        let (mut write, mut read) = ws_stream.split();

       
        let client_id = Uuid::new_v4().to_string();
        println!("New client connected: {}", client_id);

        
        let welcome_message = format!("client_id:{}", client_id);
        write.send(Message::Text(welcome_message)).await.unwrap();

        
        let clients_clone = Arc::clone(&clients);

        
        {
            let mut clients = clients.lock().await;
            clients.insert(client_id.clone(), write);
        }

        
        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        
                         println!("Received raw message: {}", text);

                        let chat_message: ChatMessage = match serde_json::from_str(&text) {
                            Ok(msg) => msg,
                            Err(_) => {
                                eprintln!("Error deserializing message");
                                break;
                            }
                        };
                        let mut clients = clients_clone.lock().await;
                        
                        if let Some(recipient) = clients.get_mut(&chat_message.to) {
                            let response = format!("Message from {}: {}", chat_message.from, chat_message.message);
                            recipient.send(Message::Text(response)).await.unwrap();
                        } else {
                            
                            if let Some(sender) = clients.get_mut(&chat_message.from) {
                                sender.send(Message::Text(format!("Recipient {} is not connected", chat_message.to))).await.unwrap();
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error processing message: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
            
        clients_clone.lock().await.remove(&client_id);
        });
    }
}
async fn serve_html() {
    let addr = ([127, 0, 0, 1], 8080).into();
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(|_req: Request<Body>| async {
            let html_content = include_str!("../index.html"); 
            Ok::<_, hyper::Error>(Response::new(Body::from(html_content)))
        }))
    });

    
    let server = Server::bind(&addr).serve(make_svc);

    println!("HTML server running at http://127.0.0.1:8080");
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
