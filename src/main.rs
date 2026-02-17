use axum::{response::Html, routing::get, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // 1. Logs: para ver que arranca
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. Rutas: Solo una, la raíz "/"
    let app = Router::new().route("/", get(handler));

    // 3. Servidor: Configurar puerto desde variable de entorno o usar 3030 por defecto
    let port = std::env::var("PORT").unwrap_or_else(|_| "3030".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("⏰ Rust Aero Clock online en http://localhost:{}", port);
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    // Incrustamos el HTML directamente en el binario final
    Html(include_str!("../index.html"))
}
