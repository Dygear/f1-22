#![allow(non_snake_case)]

use std::net::UdpSocket;
use prctl::set_name;

mod packet;

fn main() {
    set_name("Timing and Scoring").expect("Couldn't set process title.");

    let socket = UdpSocket::bind("0.0.0.0:20777").expect("Couldn't bind to address.");
    println!("UDP Port Bound");

    let mut buffer = [0; 4096];
    loop
    {
        let (_numBytes, socketAddress) = socket.recv_from(&mut buffer).unwrap();
        let packet: packet::Header = match packet::Header::unpack(&buffer)
        {
            Some(packet) => packet,
            None => { continue; }
        };
    }
}
