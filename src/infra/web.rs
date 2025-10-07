use actix_web::{App, HttpServer, web};

pub async fn run(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello world!" }))
            
    })
    .bind((addr, port))?
    .run()
    .await
}
