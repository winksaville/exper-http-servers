// #![cfg_attr(coverage_nightly, feature(no_coverage))] // Doesn't work yet
#![feature(no_coverage)]

//Based on https://gist.github.com/mjohnsullivan/e5182707caf0a9dbdf2d).

use lazy_static::lazy_static;
use regex::Regex;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_read(mut stream: &TcpStream) {
    println!("handle_read:+");
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("req_str: '{}'", req_str);
        }
        Err(e) => println!("Unable to read stream: {}", e),
    }
    println!("handle_read:-");
}

fn handle_write(mut stream: &TcpStream) {
    println!("handle_write:+");
    let response = b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
    match stream.write(response) {
        Ok(_) => println!("Response sent: '{}'", String::from_utf8_lossy(response)),
        Err(e) => println!("Failed sending response: {}", e),
    }
    println!("handle_write:-");
}

fn read_header(rdr: &mut BufReader<TcpStream>) -> (String, Vec<String>, usize) {
    println!("read_header:+ ***");
    lazy_static! {
        static ref BODY_LEN_RE: Regex = Regex::new("(?i)content-length *: *(\\d*)").unwrap();
    }

    println!("read_header: create locals");
    let mut hdr = String::new();
    let mut hdrs = Vec::<String>::new();
    let mut body_len = 0usize;
    let mut cmd = String::new();

    println!("read_header: read cmd");
    if rdr.read_line(&mut cmd).unwrap() == 0 {
        // Nothing to read assume connection is lost
        return (cmd.trim().to_owned(), hdrs, body_len);
    }
    println!("read_header: cmd='{cmd}'");

    loop {
        rdr.read_line(&mut hdr).unwrap();
        let trimed_hdr = hdr.trim();
        if trimed_hdr.is_empty() {
            println!("read_header: blank line, headers done or connection closed");
            break;
        }
        if let Some(caps) = BODY_LEN_RE.captures(trimed_hdr) {
            println!("BODY_LEN_RE match: {caps:?}");
            body_len = (caps[1]).parse().unwrap();
        }
        hdrs.push(trimed_hdr.to_string());
        hdr.clear();
    }

    println!("read_header:-");
    (cmd.trim().to_owned(), hdrs, body_len)
}

#[allow(unused)]
fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(&stream);
}

fn looping_client(stream: TcpStream) {
    println!("looping_client:+");
    let read_stream = stream.try_clone().unwrap();
    let write_stream = stream;

    let mut rdr = BufReader::new(read_stream);

    loop {
        println!("looping_client: TOL");

        // Read header
        let (cmd, hdrs, body_len) = read_header(&mut rdr);
        println!("cmd: {cmd} body_len: {body_len}");
        println!("hdrs:\n'{hdrs:#?}'");
        if cmd.is_empty() {
            println!("looping_client: exit loop");
            break;
        }

        // FTM assume we can read body in one read!!
        let mut buf = vec![0u8; body_len];
        rdr.read_exact(&mut buf).unwrap();
        println!(
            "body: len={}\n'{}'",
            buf.len(),
            String::from_utf8_lossy(&buf)
        );
        //println!("body: len={}\n'{:?}'", buf.len(), &buf);

        // Write a response
        handle_write(&write_stream);
    }
    println!("looping_client:-");
}

fn main() {
    let address: String = if let Some(addr_port) = std::env::args().nth(1) {
        addr_port
    } else {
        #[allow(clippy::iter_nth_zero)]
        let app_name = if let Some(app) = std::env::args().nth(0) {
            app
        } else {
            "httpsrvr1".to_owned()
        };
        println!("Usage: {app_name} addr:port");
        println!("  addr is an ip address like 127.0.0.1");
        println!("  port is 0..65535");
        return;
    };

    let listener = match TcpListener::bind(&address) {
        Ok(socket) => socket,
        Err(e) => {
            println!("Unable to bind to {}, err: {}", &address, e);
            return;
        }
    };
    println!("Listening for connections on addr {}", &address);

    for stream in listener.incoming() {
        println!("main: TOL for stream");
        match stream {
            Ok(stream) => {
                println!("in-comming: call spawning");
                thread::spawn(|| {
                    println!("main: spawn:+");
                    //handle_client(stream)
                    looping_client(stream);
                    println!("main: spawn:-");
                });
                println!("in-comming: retf spawning");
            }
            Err(e) => {
                println!("Unable to connect to {}, err: {}", address, e);
            }
        }
        println!("main: BOL for stream");
    }
}

#[cfg(test)]
mod test {

    //#[cfg_attr(coverage_nightly, no_coverage)] // Doesn't work yet?
    #[no_coverage]
    #[test]
    fn test_do_nothing() {}
}
