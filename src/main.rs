use actix_web::{App, get, HttpServer, Responder, web};
use fuckin_lib::my_add;

#[get("/{a}/{b}")]
async fn add_handler(web::Path((a, b)): web::Path<(i32, i32)>) -> impl Responder {
    format!("{}\n", my_add(a, b))
}

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/add")
                .service(add_handler)
        )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}