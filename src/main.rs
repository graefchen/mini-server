use std::io::{BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

/// cut both (first & last) char
fn cbc(s: String) -> String {
    if s.len() <= 2 {
        return s;
    }
    s[1..s.len() - 1].to_string()
}

/// trim slash at start and end .. if it matches
fn trim(s: String) -> String {
    s.trim_start_matches("/").trim_end_matches("/").to_owned()
}

/// thrim slash in start
fn trim_s(s: String) -> String {
    s.trim_start_matches("/").to_owned()
}

fn index_file(path: String) -> bool {
    file(path + "index.html")
}

fn file(path: String) -> bool {
    Path::new(&path).is_file()
}

fn dir(path: String) -> bool {
    Path::new(&path).is_dir()
}

fn read_file(path: String) -> Vec<u8> {
    std::fs::read(path).unwrap()
}

fn handle_request(res: String) -> Vec<u8> {
    let mut ok_message = b"HTTP/1.1 200 OK\r\n\r\n".to_vec();
    let mut error_message = b"HTTP/1.1 404 NOT FOUND\r\n\r\n".to_vec();
    if res.ends_with("/") {
        if index_file(trim_s(res.clone())) {
            ok_message.append(&mut read_file(trim_s(res.clone()) + "index.html"));
            return ok_message;
        }
        if dir(cbc(res.clone())) {
            ok_message.append(
                &mut b"<!DOCTYPE html><html lang=\"en\"><head></head><body><h1>This here should be a fancy file manager</h1></body></html>".to_vec()
            );
            return ok_message;
        }
    }
    if file(trim_s(res.clone())) {
        ok_message.append(&mut read_file(trim_s(res.clone())));
        return ok_message;
    }
    if file(trim(res.clone()) + ".html") {
        ok_message.append(&mut read_file(trim(res.clone()) + ".html"));
        return ok_message;
    }
    // 404 error
    error_message.append(
        &mut b"<!DOCTYPE html><html lang=\"en\"><head></head><body><h1>404 - NOT FOUND</h1></body></html>".to_vec(),
    );
    error_message
}

fn handle_connection(mut stream: TcpStream) {
    let mut rdr = std::io::BufReader::new(&mut stream);
    let mut l = String::new();
    rdr.read_line(&mut l).unwrap();
    match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
        ["GET", resource, "HTTP/1.1"] => {
            stream.write_all(&handle_request(resource.to_string())).unwrap();
        }
        _ => {}
    };
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream.try_clone().unwrap());
    }
}
