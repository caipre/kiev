extern crate regex;

use std::collections::HashMap;
use std::net::TcpListener;
use std::io::{BufReader,BufWriter};
use std::io::prelude::*;

use regex::Regex;

fn main() {
    let respond = |status, body: &str| -> String {
        format!("HTTP/1.1 {status}\r\n\
                 Content-Type: text/plain\r\n\
                 Content-Length: {length}\r\n\
                 Connection: close\r\n\r\n\
                 {body}\n", status=status, body=body, length=body.len()+1)
    };

    let mut map: HashMap<String, String> = HashMap::new();
    let get_re = Regex::new(r"GET /get\?key=(?P<key>\S+) HTTP/1.1").unwrap();
    let set_re = Regex::new(r"GET /set\?(?P<key>[^=]+)=(?P<val>\S+) HTTP/1.1").unwrap();
    let server = TcpListener::bind("localhost:4000").unwrap();
    for connection in server.incoming() {
        match connection {
            Err(_) => panic!("connection failure"),
            Ok(stream) => {
                let mut writer = BufWriter::new(stream.try_clone().unwrap());
                let mut reader = BufReader::new(stream);
                let mut request = String::new();
                reader.read_line(&mut request);
                if let Some(captures) = get_re.captures(&request) {
                    match map.get(captures.name("key").unwrap()) {
                        Some(ref val) => write!(writer, "{}", respond(200, val)),
                        None => write!(writer, "{}", respond(400, "error: no such key")),
                    };
                } else if let Some(captures) = set_re.captures(&request) {
                    let k = captures.name("key").unwrap();
                    let v = captures.name("val").unwrap();
                    map.insert(k.to_owned(), v.to_owned());
                    write!(writer, "{}", respond(200, format!("{}: {}", k, v).as_ref()));
                } else {
                    write!(writer, "{}", respond(400, "error: unknown method"));
                }
            }
        }
    }
}
