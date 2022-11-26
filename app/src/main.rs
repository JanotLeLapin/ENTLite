use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide at least 2 arguments: <email> <password>");
    }
    let client = api::auth::login(args.get(1).unwrap(), args.get(2).unwrap()).await.unwrap();
}