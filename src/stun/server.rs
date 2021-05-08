use std::sync::mpsc::Sender;
use std::fmt::Error;
use rustun::server::{BindingHandler, UdpServer};
use fibers_global;

fn stun_server(port: i32) {
    let addr = format!("0.0.0.0:{}", port).parse().unwrap();
    let server = UdpServer::start(
        fibers_global::handle(),
        addr,
        BindingHandler,
    );

    fibers_global::execute(server);
}

pub fn start_stun_server(port: i32) -> Result<(), Error> {
    // port1
    stun_server(port);
    // port2
    stun_server(port + 1);

    Ok(())
}