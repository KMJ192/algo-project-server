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
	let addr = match env::var("SERVER_HOST") {
		Ok(host) => host,
		Err(_e) => "0.0.0.0:8000".to_string(),
	};


	HttpServer::new(|| {
		App::new()
			.service(hello)
			// .service(echo)
			.service(random_matching_build)
	})
	.bind(&addr)?
	.run()
	.await
}