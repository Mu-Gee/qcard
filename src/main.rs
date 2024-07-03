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


    // Will process form data later

    /* // For front end debugging uncomment this block to return user details to the console(on the browser)
    let response = format!(
        "Received details: Name: {}, Phone: {}, Email: {}",
        form.name, form.phone, form.email
    );
    // Send a JSON response back to the client
    HttpResponse::Ok().json(json!({"status": "success", "message": response}))
    */

    // Should above block be uncommented comment this out
    HttpResponse::Ok().json(json!({"status": "success", }))
}

#[get("/submit-details")]
async fn submit_details_get() -> impl Responder {
    HttpResponse::Ok().body("Nice try this endpoint is for form submissions only.")
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
            .service(generate_qr)
            .service(submit_details)
            .service(submit_details_get)
            .route("/hey", web::get().to(manual_hello))
            .service(Files::new("/", "./FrontEnd").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}