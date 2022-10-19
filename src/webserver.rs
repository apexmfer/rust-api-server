
use warp::Filter;



//https://lib.rs/crates/warp


/*

http://127.0.0.1:3030/hello/brochacho
->  Hello,brochacho!  

*/

#[tokio::main]
pub async fn start() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    println!("starting web server");

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

