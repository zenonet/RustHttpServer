use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{IpAddr, SocketAddr, TcpListener};

fn main() {
    println!("Starting server...");
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::from([0, 0, 0, 0]), 80))
        .expect("TcpListener should be binable");
    println!("Server started!");
    let mut message: String = String::from("Nobody has sent a message yet :(");
    let mut counter = 0;
    let mut msg_log = File::create("msglog.txt").unwrap();
    loop {
        let (sock_stream, client_addr) = listener.accept().expect("TODO: panic message");
        {
            let start = std::time::Instant::now();

            let mut input_stream = BufReader::new(&sock_stream);
            let mut output_stream = BufWriter::new(&sock_stream);

            let Ok(fill_res) = input_stream.fill_buf() else {
                println!("Failed to read request data, aborting request");
                continue;
            };

            let request = String::from_utf8_lossy(fill_res);
            let Some(status_line) = request.lines().next() else {
                println!("Failed to read request data, aborting request");
                continue;
            };

            let mut did_msg_change = false;
            if let Some(slash_index) = status_line.find('/') {
                let path = &status_line[slash_index..];
                if path.starts_with("/msg/") {
                    if let Some(end) = path[5..].find(' ') {
                        let msg = String::from(&path[5..end + 5]);
                        if msg.len() > 0 {
                            if validate_msg(&*msg) {
                                message = msg;
                                println!("Got new message: '{message}'");
                                let _ = msg_log.write(format!("{message}\n").as_bytes());
                                did_msg_change = true;
                            }else{
                                println!("Slur filter blocked message: '{msg}'")
                            }
                        }
                    }
                }
            }

            println!("got new client and data: {}", request);

            let inner_content = if did_msg_change {
                "Thanks for leaving a message!"
            } else {
                &*format!("A previous visitor left a message here for you:\n\
                        {message}\n\n\
                        You can send a message to the next visitor by sending a request like this: https://lx.zenonet.de:8874/msg/YourMessageHere\n\
                        If you send one, pls be nice\n\
                        If you do this I will (obviously) memorize your message and I'll take a hashcode of your ip address and keep it in memory until the next visitor writes a message")
            };

            let counter_as_string = counter.to_ordinal_number();
            let content = format!("Hi there!\n\
                        This is my little http server written in Rust.\n\
                        I am feeding it through a reverse proxy for SSL though\
                        \n\n\
                        {inner_content}\n\n\
                        Also, you're the {counter_as_string} visitor (or atleast this is the {counter_as_string} request since I started the server)");
            counter += 1;

            let content_length = content.len() + 1;
            let _ = output_stream.write(format!("HTTP/1.0 200 OK\nContent-Length:{content_length}Content-Type: text/plain\n\n{content}\n").as_bytes());
            let end = std::time::Instant::now();
            println!(
                "Request took {}ms",
                (end - start).as_micros() as f32 / 1000f32
            );
        }
    }
}

fn validate_msg(msg: &str) -> bool {
    let file = File::open("msg_blacklist.txt").unwrap();

    for slur in BufReader::new(file).lines() {
        if slur.is_ok() && msg.contains(&slur.unwrap()){
            return false;
        }
    }

    !msg.contains('.') && !msg.starts_with("http") && !msg.contains("www") && !msg.contains("@")
}

trait NumberFormatting {
    fn to_ordinal_number(&self) -> String;
}
impl NumberFormatting for i32 {
    fn to_ordinal_number(&self) -> String {
        if *self == 1 {
            String::from("1st")
        } else if *self == 2 {
            String::from("2nd")
        } else if *self == 3 {
            String::from("3rd")
        } else {
            format!("{self}th")
        }
    }
}
