use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}

fn calculate_pi(iterations: u64) -> f64 {
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