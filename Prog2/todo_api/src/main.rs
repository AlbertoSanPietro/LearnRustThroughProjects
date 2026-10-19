use actix_web::{App, HttpResponse, HttpServer, Result, web};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: String,
    title: String,
    description: String,
    completed: bool,
    created_at: String,
}

type TaskStorage = Mutex<HashMap<String, Task>>;

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "healthy",
            "timestap": chrono::Utc::now().to_rfc3339()
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let task_storage = web::Data::new(TaskStorage::new((HashMap::new())));

    println!("Starting server on 8080\n");

    HttpServer::new(move || {
        App::new()
            .app_data(task_storage.clone())
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
