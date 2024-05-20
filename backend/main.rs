use route;
#[tokio::main]
async fn main() {
    println!("🚀 Axum - NextJS - App v1.0");
    println!("🧓 SunsJay <sunsjay0806@gmail.com>");
    let app = route::route();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
