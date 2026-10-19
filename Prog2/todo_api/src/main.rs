use actix_web::{App, HttpResponse, HttpServer, Result, web};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: String,
    title: String,
    description: String,
    completed: bool,
    created_at: String,
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "healthy",
            "timestap": chrono::Utc::now().to_rfc3339()
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on 8080\n");

    HttpServer::new(|| App::new().route("/health", web::get().to(health_check)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
