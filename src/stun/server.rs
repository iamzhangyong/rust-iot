use std::sync::mpsc::Sender;
use std::fmt::Error;
use rustun::server::{BindingHandler, UdpServer};
use fibers_global;

pub fn start_stun_server(port: i32) -> Result<(), Error> {
    let addr = format!("0.0.0.0:{}", port).parse().unwrap();
    UdpServer::start(
        fibers_global::handle(),
        addr,
        BindingHandler,
    );
    Ok(())
}