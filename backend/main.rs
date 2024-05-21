use route::route;

#[tokio::main]
async fn main() {
    println!("🚀 Axum - NextJS - App v1.0");
    println!("🧓 SunsJay <sunsjay0806@gmail.com>");
    let app = route();
    //    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => axum::serve(listener, app).await.unwrap(),
        Err(e) => {
            println!("{}", e);
        }

    }
}
