use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    // Prepare an example HTTP POST request
    let data = r#"{
        "machine_hostname": "localhost",
        "machine_ip": 123456,
        "machine_open_ports": [80, 443],
        "bitcoin_addresses_found": ["address1", "address2"],
        "email_addresses_found": ["email1@example.com", "email2@example.com"],
        "hcode": "i3no2nro32nro2nf43nfe2nf2f@8@N#n3on2"
    }"#;

    let content_length = data.len();
    let request = format!(
        "POST / HTTP/1.1\r\n\
        Host: 127.0.0.1:8080\r\n\
        Content-Type: application/json\r\n\
        Content-Length: {}\r\n\
        \r\n\
        {}",
        content_length, data
    );

    // Send the HTTP request to the server
    stream.write(request.as_bytes())?;

    // Read the server's response
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Print the server's response
    println!("Server Response:\n{}", response);

    Ok(())
}
