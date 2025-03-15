use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::time::Instant;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap(); //Establish socket

    for stream in listener.incoming() { //Listen for requests
        let stream = stream.unwrap(); //Server socket + Incoming socket + identifier
        println!("{:?}", stream);
        handle_connection(&stream);
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let start = Instant::now(); //Time counter

    let http_request = read_stream(stream);

    println!("Request: {http_request:#?}");

    let request_line = &http_request[0];
    let response: String = if request_line.starts_with("GET /pi/") {
        let iterations = extract_param(request_line);
        if let Some(iterations) = iterations {
            let pi_value = calculate_pi(iterations);
            let duration = start.elapsed().as_secs_f64();
            format!("HTTP/1.1 200 OK\r\n\r\nValor de Pi para el termino {}: {} ({:?}s)", iterations, pi_value, duration)
        } else {
            "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
        }
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
    };

    stream.write_all(response.as_bytes()).unwrap()
}

fn read_stream(stream: &TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    http_request
}

fn extract_param(request: &String) -> Option<u128> {
    let split_message: Vec<&str> = request.split_whitespace().collect();
    println!("{:?}", split_message);
    let path = split_message[1];

    if let Some(id_str) = path.strip_prefix("/pi/") {
        if let Ok(iterations) = id_str.parse::<u128>() {
            return Some(iterations)
        }
    }

    return None
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