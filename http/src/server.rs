use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn init(host: &str, port: i16) {
    let listener: TcpListener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();

    for stream in listener.incoming() {
        //let temp = stream.
        let unwrapped_stream = stream.unwrap();
        // let temp : = unsafe { stream }

        handle_connection(unwrapped_stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf: [u8; 4096] = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}
