use std::{
    self, fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3242").unwrap();

    for stream in listener.incoming() {
        println!("New stream received!");
        let stream = stream.unwrap();
        generate_response(stream);
    }
}

fn generate_response(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).unwrap();
    let request_line = String::from_utf8_lossy(&buffer[..size]);
    println!("Request received!: {}", &request_line);
    // HTTP Request:
    // 1: Method Request-URI HTTP-Version CRLF
    // 2: headers CRLF
    // 3: message-body
    // let http_request: Vec<_> = buf_reader
    // .lines()
    // .map(|result| result.unwrap())
    // .take_while(|line| !line.is_empty())
    // .collect();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "pages/hello.html"),
        // A simulated slow response
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "pages/sleep.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "pages/404.html"),
    };
    // HTTP Response:
    // 1: HTTP-Version Status-Code Reason-Phrase CRLF
    // 2: headers CRLF
    // 3: message-body

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Sent response!");
}
