mod qr;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use qr::qrgen;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[get("/generate_qr")]
async fn generate_qr() -> impl Responder {
    qrgen();
    HttpResponse::Ok().body("QR code generated and saved as contact_qr_code.png")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(generate_qr)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}