mod random_matching;

// use std::env;
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
	// let mut bind_addr = env::var("BIND_ADDR").expect("Expected a port in the environment");
	// if bind_addr.len() == 0 {
	//   bind_addr = String::from("localhost:8080");
	// }

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