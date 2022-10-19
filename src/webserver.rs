
use warp::Filter;



//https://lib.rs/crates/warp
//https://www.youtube.com/watch?v=j2dy0xb1618


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


    let post_items = warp::post()
    .and(warp::path("v1"))
    .and(warp::path("groceries"))
    .and(warp::path::end())
    .and(json_body())
    .and(store_filter.clone())
    .and_then(add_grocery_list_item);

    let get_items = warp::get()
    .and(warp::path("v1"))
    .and(warp::path("groceries"))
    .and(warp::path::end())
    .and(store_filter.clone())
    .and_then(get_grocery_list);


    //load the routes from the controllers 
    
    let routes = add_items.or(post_items);


    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

