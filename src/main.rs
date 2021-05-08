mod stun;

use std::sync::mpsc::channel;
use clap::{App, Arg};
use actix_web::{web, App as WebApp, HttpRequest, HttpServer, Responder};
use crate::stun::server::*;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = App::new("rust-iot").arg(
        Arg::with_name("stun_port").
            long("stun_port").
            takes_value(true).
            required(true).
            default_value("3478")
    );

    let matches = app.get_matches();
    let stun_port = matches.value_of("stun_port").unwrap();

    let stun_port = stun_port.parse::<i32>().unwrap();

    let (sender, receiver) = channel();
    start_stun_server(stun_port);

    HttpServer::new(|| {
        WebApp::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;

    let ret: i32 = receiver.recv().unwrap();
    println!("rust-iot exit: {}", ret);

    Ok(())
}
