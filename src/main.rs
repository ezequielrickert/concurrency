use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::time::Instant;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let start = Instant::now();

    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    let request_line = &http_request[0];
    let response: String = if request_line.starts_with("GET /pi/") {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() > 1 {
            let path = parts[1];
            if let Some(id_Str) = path.strip_prefix("/pi/") {
                if let Ok(id) = id_Str.parse::<u128>() {
                    let pi_value = calculate_pi(id);
                    let duration = start.elapsed().as_secs_f64();
                    format!("HTTP/1.1 200 OK\r\n\r\nValor de Pi para el termino {}: {} ({:?}s)", id, pi_value, duration)
                } else {
                    format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", path)
                }
            } else {
                format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", path)
            }
        } else {
            format!("HTTP/1.1 200 OK\r\n\r\n{}", "")
        }
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
    };

    stream.write_all(response.as_bytes()).unwrap()
}

fn calculate_pi(iterations: u128) -> f64 {
    let mut pi = 0.0;
    let mut denominator = 1.0;
    let mut sign = 1.0;

    for _ in 0..iterations {
        pi += sign / denominator;
        denominator += 2.0;
        sign *= -1.0;
    }

    pi * 4.0
}