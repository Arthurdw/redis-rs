use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_message(_message: &str) -> &str {
    "+PONG\r\n"
}

fn handle_connetion(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut data: [u8; 512] = [0; 512];

    loop {
        let n = stream.read(&mut data)?;

        if n == 0 {
            break;
        }

        let content: &str = str::from_utf8(&data[0..n])?;
        let response = handle_message(content);
        stream.write_all(response.as_ref())?;
    }

    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connetion(stream).expect("Something went wrong in the connection"),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
