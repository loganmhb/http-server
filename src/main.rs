#![feature(slice_patterns)]
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::vec::Vec;
use std::collections::HashMap;


fn parse_header(header: &str) -> Result<(&str, &str), String> {
    let iter: Vec<&str> = header.split(": ").collect();
    match iter.as_slice() {
        [header_name, header_value] =>
            Ok((header_name, header_value)),
        [""] => Ok(("","")), // FIXME: return a more informative type
        _ => Err(format!("Failed to parse header '{}'", header))
    }
}

static BAD_REQUEST: &'static str = "HTTP/1.1 400 Bad Request";
static HELLO_WORLD: &'static str = "HTTP/1.1 200 OK\n\nHello world!";

fn handle_request(mut stream: &mut TcpStream) {
    let mut reader = BufReader::new(&mut stream);
    let mut headers: HashMap<String, String> = HashMap::new();

    // Read the HTTP headers into a hash map
    {
        // new scope needed in order to make sure we'll still have access to reader later
        let mut lines = (&mut reader).lines();
        let request_line = lines.next().unwrap().unwrap();
        println!("Request line: {}", request_line);

        for line in lines {
            match parse_header(&line.unwrap()) {
                Ok(("","")) => break, // end of headers
                Ok((k, v)) => {
                    println!("Parsed header. Name: {}, value: {}", k, v);
                    headers.insert(k.to_string(), v.to_string());
                    },
                Err(msg) => { panic!["Bad header: {}", msg] }
            };
        }
    }
    println!("Done parsing headers.");
    let mut writer = reader.get_mut();
    writer.write(HELLO_WORLD.as_bytes());
    ()
}

fn main() {
    let port = 8000;
    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
    println!("Listening for HTTP requests on port {}", port);

    for stream in listener.incoming() {
        thread::spawn(move || {
            handle_request(&mut stream.unwrap());
        });
    }
}
