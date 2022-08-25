// #![cfg_attr(coverage_nightly, feature(no_coverage))] // Doesn't work yet
#![feature(no_coverage)]

//Based on https://gist.github.com/mjohnsullivan/e5182707caf0a9dbdf2d).

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("req_str: '{}'", req_str);
        }
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    match stream.write(response) {
        Ok(_) => println!("Response sent: '{}'", String::from_utf8_lossy(response)),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}

fn main() {
    let port: String = if let Some(port) = std::env::args().nth(1) {
        port
    } else {
        #[allow(clippy::iter_nth_zero)]
        let app_name = if let Some(app) = std::env::args().nth(0) {
            app
        } else {
            "httpsrvr1".to_owned()
        };
        println!("Usage: {app_name} port");
        println!("  port: 0..65535");
        return;
    };
    println!("args.len = {:?}", std::env::args().nth(1));

    let ipv4 = "127.0.0.1";
    let addr = format!("{ipv4}:{port}");
    let listener = match TcpListener::bind(&addr) {
        Ok(socket) => socket,
        Err(e) => {
            println!("Unable to bind to {}, err: {}", &addr, e);
            return;
        }
    };
    println!("Listening for connections on addr {}", &addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Unable to connect to {}, err: {}", addr, e);
            }
        }
    }
}

#[cfg(test)]
mod test {

    //#[cfg_attr(coverage_nightly, no_coverage)] // Doesn't work yet?
    #[no_coverage]
    #[test]
    fn test_do_nothing() {}
}
