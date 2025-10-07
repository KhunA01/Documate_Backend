use backend::infra::web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenvy::dotenv().ok();

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "5001".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let addr: String  = std::env::var("ADDR")
        .unwrap_or_else(|_| "0.0.0.0".to_string());

    web::run(&addr, port).await
}
