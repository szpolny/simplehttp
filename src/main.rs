use std::{net::TcpListener, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|e| {
        eprint!("Failed to bind to address: {}", e);
        std::process::exit(1);
    });

    println!("Server listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");

                thread::spawn(move || {});
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
