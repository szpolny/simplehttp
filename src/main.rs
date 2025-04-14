use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

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

                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => {
            eprintln!("Failed to read request line: {}", e);
            return;
        }
        None => {
            eprintln!("No request line found");
            return;
        }
    };

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "html/hello.html")
    } else if request_line == "GET /sleep HTTP/1.1" {
        println!("Handling /sleep request, sleeping for 5 seconds...");
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "html/404.html")
    };

    let contents = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", filename, e);
            let error_response = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Length: 0\r\n\r\n";
            if let Err(write_err) = stream.write_all(error_response.as_bytes()) {
                eprintln!("Failed to send 500 error response: {}", write_err);
            }
            if let Err(flush_err) = stream.flush() {
                eprintln!("Failed to flush stream after 500 error: {}", flush_err);
            }
            return;
        }
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    match stream.write_all(response.as_bytes()) {
        Ok(_) => println!("Response sent successfully."),
        Err(e) => eprintln!("Failed to send response: {}", e),
    }

    match stream.flush() {
        Ok(_) => println!("Stream flushed."),
        Err(e) => eprintln!("Failed to flush stream: {}", e),
    }

    println!("Connection handling finished.");
}
