mod stun;

use std::sync::mpsc::channel;
use clap::{App, Arg};
use crate::stun::server::*;

fn main() {
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

    let ret: i32 = receiver.recv().unwrap();
    println!("rust-tool exit: {}", ret);
}
