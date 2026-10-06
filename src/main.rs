use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;
use std::io::Read;
use std::fs::OpenOptions;

fn append(entry: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("db.log")
        .unwrap();
    writeln!(file, "{}", entry).unwrap();
}

fn load() -> HashMap<String, String> {
    let mut store = HashMap::new();
    if let Ok(content) = std::fs::read_to_string("db.log"){
        for line in content.lines() {
            let mut parts = line.splitn(2, ' ');
            let op = parts.next().unwrap_or("");
            let rest = parts.next().unwrap_or("")
            match op {
                "SET" => {
                    let mut p = rest.splitn(2, ' ');
                    let key = p.next().unwrap_or("").to_string();
                    let value = p.next().unwrap_or("").to_string();
                    store.insert(key, value);
                }
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on 127.0.0.1:8080");
    let mut store:  HashMap<String, String> = HashMap::new();

    store.insert("name".to_string(), "lewis".to_string());

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut reader = BufReader::new(&stream);

       let mut lines = (&mut reader).lines();
       let request_line = lines.next().unwrap().unwrap();
       println!("Request line: {}", request_line);
  

       let mut parts = request_line.split_whitespace();
       let method = parts.next().unwrap_or("");
       let path = parts.next().unwrap_or("");

       println!("Methods: {}, Path: {}", method, path);

       let mut content_length: usize = 0;

       for line in lines {
           let line = line.unwrap();
           if line.is_empty() {
               break;

           }
            if line.to_lowercase().starts_with("content-length:") {
                let value = line.split(':').nth(1).unwrap().trim();
                content_length = value.parse().unwrap_or(0);
            }
        }

        println!("Content-Length: {}", content_length);

        let mut body = vec![0; content_length];
        reader.read_exact(&mut body).unwrap();

        let body = String::from_utf8_lossy(&body).to_string();
        println!("Body: {}", body);

        let response = if method =="POST" && path == "/set" {
            if let Some((key, value)) = body.split_once('='){
                store.insert(key.to_string(), value.to_string());
                let msg = "OK";
                format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", msg.len(), msg)
            } else {
                let msg = "Bad request";
                format!("HTTP/1.1 400 Bad Request\r\nContent-Length: {}\r\n\r\n{}", msg.len(), msg)

            }   
        } else if path.starts_with("/get/") {
            let key = path.strip_prefix("/get/").unwrap();
            match store.get(key) {
                Some(value) => format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", value.len(), value),
     
                None => {
                    let body = "Key not found";
                    format!("HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",body.len(), body)
                 }
            }
        } else {
                let body = "404 Not Found";
                format!("HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}", body.len(), body)
            };
        stream.write_all(response.as_bytes()).unwrap();
    }
}
