use std::thread;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;

mod extensiblemongo;
mod webserver;
//https://doc.rust-lang.org/std/thread/fn.spawn.html

const serverConfigPath:&str = "config/serv.json";

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}




fn main() {
    println!("Hello, world!");

   

        let handler = thread::spawn(|| {
            extensiblemongo::start();
        }
    );
 

   
    let data = fs::read_to_string(serverConfigPath).expect("Unable to read file");

    let p: Person = serde_json::from_str(&data).expect("unable to parse");

    webserver::start();

    println!("start inf loop");
    //infinite loop
    loop {


    }
}
   


