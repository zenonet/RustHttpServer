use std::fmt::format;
use std::io::{BufReader, BufWriter};
use std::net::{IpAddr, Shutdown, SocketAddr, SocketAddrV4, TcpListener, ToSocketAddrs};
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::from([0, 0, 0, 0]), 80)).unwrap();

    let mut counter = 0;
    loop {
        let (mut sock_stream, client_addr) = listener.accept().expect("TODO: panic message");
        {
            let start = std::time::Instant::now();

            let mut input_stream = BufReader::new(&sock_stream);
            let mut output_stream = BufWriter::new(&sock_stream);

            let fill_res = input_stream.fill_buf();
            if fill_res.is_err(){
                println!("Failed to read request data, aborting request");
                continue
            }

            let request = String::from_utf8_lossy(fill_res.unwrap());

            println!("got new client and data: {}", request);

            let content = format!("Asbest!\nAlso, you're the {counter}th visitor");
            counter += 1;

            let content_length = content.len()+1;
            output_stream.write(format!("HTTP/1.0 200 OK\nContent-Length:{content_length}Content-Type: text/html\n\n{content}\n").as_bytes()).unwrap();
            /*        output_stream.write("HTTP/1.0 200 OK\n".as_bytes()).unwrap();
                    output_stream.write("Content-Type: text/plain\n\n".as_bytes()).unwrap();
                    output_stream.write("Yeah\n".as_bytes()).unwrap();*/
            //output_stream.flush().unwrap();

            let end = std::time::Instant::now();
            println!("Request took {}ms", (end-start).as_micros() as f32/1000f32);
        }
        //sock_stream.flush().unwrap();
        //sock_stream.shutdown(Shutdown::Both).unwrap();
        }
}
