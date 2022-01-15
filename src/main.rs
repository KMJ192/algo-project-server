mod random_matching;

use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use random_matching::matching_algorithm::random_matching_build;

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("hello")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
	HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// let app_host = env::var("APP_HOST").expect("Host not found.");
	// let app_port = env::var("APP_PORT").expect("Port not found.");
	// let db_url = env::var("DB_URL").expect("db url not found.");
	// let app_url = format!("{}:{}", &app_host, &app_port);

	HttpServer::new(|| {
		App::new()
			.service(hello)
			// .service(echo)
			.service(random_matching_build)
	})
	.bind("localhost:8080")?
	.run()
	.await
}