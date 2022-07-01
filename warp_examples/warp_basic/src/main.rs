use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::any().map(|| "Hello world!");
    // let hello = warp::path!("hello" / String).map(|name| format!("Hello {}!", name));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
