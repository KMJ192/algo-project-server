mod random_matching;

use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("hello")
}

#[get("/")]
async fn primary() -> impl Responder {
	HttpResponse::Ok().body("primary")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let addr = match env::var("SERVER_HOST") {
		Ok(host) => host,
		Err(_e) => "127.0.0.1:8080".to_string(),
	};

	HttpServer::new(|| {
		App::new()
			.service(hello)
			.service(primary)
	})
	.bind(&addr)?
	.run()
	.await
}