use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let reader = BufReader::new(&stream);

       let mut lines = reader.lines();
       let request_line = lines.next().unwrap().unwrap();
       println!("Request line: {}", request_line);
  

       let mut parts = request_line.split_whitespace();
       let method = parts.next().unwrap_or("");
       let path = parts.next().unwrap_or("");

       println!("Methods: {}, Path: {}", method, path);



       for line in lines {
           let line = line.unwrap();
           if line.is_empty() {
               break;

           }
        }
 
        let response = if path.starts_with("/get/") {
            let key = path.strip_prefix("/get/").unwrap();
            format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",key.len(), key)
        } else {
                let body = "404 Not Found";
                format!("HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}", body.len(), body)
            };
        stream.write_all(response.as_bytes()).unwrap();
     }
}
