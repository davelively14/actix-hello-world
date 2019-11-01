use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Good luck!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world, basic!")
}

#[get("/macros")]
fn index3() -> impl Responder {
    HttpResponse::Ok().body("This is with macros!")
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/basic", web::get().to(index2))
            .service(index3)
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
