use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use google_cloud_pubsub::client::{Client, ClientConfig};
use tera::{Tera, Context};
use serde::Deserialize;
use std::env;
use base64::{engine::general_purpose, Engine as _};
use tokio::sync::mpsc;
use google_cloud_pubsub::subscription::ReceiveConfig;
use tokio_util::sync::CancellationToken;


#[derive(Deserialize, Debug)]
struct PubSubMessage {
    data: String,
}

#[derive(Deserialize, Debug)]
struct PubSubPayload {
    message: PubSubMessage,
}

#[get("/")]
async fn index() -> impl Responder {
    let project_id = env::var("PROJECT_ID").expect("PROJECT_ID not set");
    let subscription_id = env::var("SUBSCRIPTION_ID").expect("SUBSCRIPTION_ID not set");
    let config = ClientConfig::default();
    let client = Client::new(config).await.unwrap();

    let subscription_name = format!("projects/{}/subscriptions/{}", project_id, subscription_id);
    let sub = client.subscription(&subscription_name);

    let (tx, mut rx) = mpsc::channel::<String>(100);

    tokio::spawn(async move {
        let receive_config = ReceiveConfig {
            ..Default::default()
        };
        sub.receive(
            move |message, _cancellation_token| {
                let tx = tx.clone();
                async move {
                    let payload: Result<PubSubPayload, serde_json::Error> =
                        serde_json::from_slice(message.message.data.as_slice());
                    match payload {
                        Ok(payload) => {
                            let decoded_data = general_purpose::STANDARD.decode(payload.message.data).unwrap();
                            let message_string = String::from_utf8(decoded_data).unwrap();
                            let _ = tx.send(message_string).await;
                        }
                        Err(e) => {
                            println!("Failed to deserialize message: {}", e);
                        }
                    }
                    message.ack().await;
                }
            },
            CancellationToken::new(),
            Some(receive_config),
        ).await.unwrap();
    });

    let mut messages = Vec::new();
    for _ in 0..10 {
        if let Some(message) = rx.recv().await {
            messages.push(message);
        } else {
            break;
        }
    }

    let mut context = Context::new();
    context.insert("messages", &messages);

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Tera parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
    let rendered_html = tera.render("index.html", &context).unwrap();

    HttpResponse::Ok().content_type("text/html").body(rendered_html)
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(healthz)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
