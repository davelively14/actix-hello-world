use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,
}

fn index(data: web::Data<AppState>) -> impl Responder {
    let resp = format!("Good luck, {}!", &data.app_name);

    HttpResponse::Ok().body(resp)
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
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .route("/", web::get().to(index))
            .route("/basic", web::get().to(index2))
            .service(index3)
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
