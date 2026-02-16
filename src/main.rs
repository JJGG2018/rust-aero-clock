use axum::{
    response::Html,
    routing::get,
    Router,
};
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

    // 3. Servidor: Escuchar en 0.0.0.0:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("⏰ Rust Aero Clock online en http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    // Incrustamos el HTML directamente en el binario final
    Html(include_str!("../index.html"))
}
