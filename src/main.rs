use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let port = std::env::var("PORT")
        .unwrap_or("8080".into())
        .parse().unwrap();

    warp::serve(hello)
        .run(([0, 0, 0, 0], port))
        .await;
}
