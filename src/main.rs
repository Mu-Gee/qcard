mod qr;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;
use qr::qrgen;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct FormData {
    name: String,
    phone: String,
    email: String,
}

#[post("/submit-details")]
async fn submit_details(form: web::Json<FormData>) -> impl Responder {

    // This is for debugging on the rust side
    println!("Received form data: {:?}", form);

    qrgen(&form.name, &form.phone, &form.email);


    // Will process form data later
    HttpResponse::Ok().json(json!({"status": "success", }))
}

#[get("/submit-details")]
async fn submit_details_get() -> impl Responder {
    HttpResponse::Ok().body("Nice try this endpoint is for form submissions only.")
}

/* This endpoint is for testing the qrgen
#[get("/generate_qr")]
async fn generate_qr() -> impl Responder {
    qrgen("Jane Doe", "+1234567890", "janedoe@found.com");
    HttpResponse::Ok().body("QR code generated and saved as contact_qr_code.png")
}
*/

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //.service(generate_qr)
            .service(submit_details)
            .service(submit_details_get)
            .route("/hey", web::get().to(manual_hello))
            .service(Files::new("/", "./FrontEnd").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}