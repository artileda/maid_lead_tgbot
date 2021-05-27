extern crate tokio;
use std::env;

//use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TG_TOKEN").expect("Need TG_TOKEN---");
    println!("{}",token);
    Ok(())
}
