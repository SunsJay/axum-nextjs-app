use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    println!("🚀 Axum - NextJS - App v1.0");
    println!("🧓 SunsJay <sunsjay0806@gmail.com>");
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
