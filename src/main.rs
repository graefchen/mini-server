use std::io::{BufRead, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    for mut stream in listener.incoming().flatten() {
        let mut rdr = std::io::BufReader::new(&mut stream);
        let mut l = String::new();
        rdr.read_line(&mut l).unwrap();
        match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", resource, "HTTP/1.1"] => {
                loop {
                    let mut l = String::new();
                    rdr.read_line(&mut l).unwrap();
                    if l.trim().is_empty() {
                        break;
                    }
                    print!("{l}")
                }
                let mut p = std::path::PathBuf::new();
                p.push(resource.trim_start_matches("/"));
                if resource.ends_with("/") {
                    p.push("index.html");
                }
                if resource.starts_with("../") {
                    break;
                }
                stream.write_all(b"HTTP/1.1 200\r\n\r\n").unwrap();
                stream.write_all(&std::fs::read(p).unwrap()).unwrap();
            }
            _ => todo!(),
        };
    }
}
