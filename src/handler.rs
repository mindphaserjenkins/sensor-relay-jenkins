use crate::{ws, Client, Clients, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, ws::Message, Reply};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    topics: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SensorReport {
    reporter: String,
    topic: String,
    sensors: HashMap<String, String>,
}

pub async fn publish_handler(report: SensorReport, clients: Clients) -> Result<impl Reply> {
    clients
        .read()
        .await
        .iter()
        .filter(|(_, client)| client.topics.contains(&report.topic))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let serialized = serde_json::to_string(&report).unwrap();
                let _ = sender.send(Ok(Message::text(serialized)));
            }
        });

    Ok(StatusCode::OK)
}

pub async fn register_handler(request: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    let uuid = Uuid::new_v4().simple().to_string();

    register_client(uuid.clone(), request.topics, clients).await;
    Ok(json(&RegisterResponse {
        id: uuid
    }))
}

async fn register_client(id: String, topics: Vec<String>, clients: Clients) {
    clients.write().await.insert(
        id,
        Client {
            topics,
            sender: None,
        },
    );
}

pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
    clients.write().await.remove(&id);
    Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
    let client = clients.read().await.get(&id).cloned();
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}
