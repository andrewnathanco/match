use actix_files as fs;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate<'a> {
    pub(crate) name: &'a str,
}

#[get("/")]
async fn hello() -> impl Responder {
    let tmpl = HelloTemplate { name: "test" };
    let body = tmpl.render().unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "static/").use_last_modified(true))
            .service(hello)
    })
    .bind(("10.0.0.42", 1300))?
    .run()
    .await
}
