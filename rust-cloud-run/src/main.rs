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

use actix_web::error::Error as ActixError;

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let project_id = env::var("PROJECT_ID").map_err(|_| MyError("PROJECT_ID not set".to_string()))?;
    let subscription_id = env::var("SUBSCRIPTION_ID").map_err(|_| MyError("SUBSCRIPTION_ID not set".to_string()))?;
    let config = ClientConfig::default();
    let client = Client::new(config).await.map_err(|e| MyError(format!("Failed to create client: {}", e)))?;

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
                    Ok::<(), MyError>(())
                }
            },
            CancellationToken::new(),
            Some(receive_config),
        ).await.map_err(|e| MyError(format!("Failed to receive messages: {}", e))).ok();
    });

    let mut messages = Vec::new();
    use tokio::time::{timeout, Duration};

    for _ in 0..10 {
        match timeout(Duration::from_secs(1), rx.recv()).await {
            Ok(Some(message)) => {
                messages.push(message);
            }
            _ => {
                break;
            }
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
    let rendered_html = tera.render("index.html", &context).map_err(|e| format!("Failed to render template: {}", e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered_html))
}

use actix_web::{error, Error};

#[derive(Debug)]
struct MyError(String);

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::convert::From<String> for MyError {
    fn from(value: String) -> Self {
        MyError(value)
    }
}

impl error::ResponseError for MyError {}

#[get("/healthz")]
async fn healthz() -> Result<HttpResponse, MyError> {
    println!("Health check called!");
    Ok(HttpResponse::Ok().body("OK"))
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
