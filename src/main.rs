use dope_db::server;

#[tokio::main]
async fn main() {
    let router = server::get_router().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
