use std::io::{stdin, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

// utiliser clap

fn server0() {
    println!("Server");
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("...");
    match listener.accept() {
        Ok((client, adr)) => {
            println!("alu {}:{}", adr.ip(), adr.port());
            println!("client {}", client.local_addr().unwrap());
        }
        Err(e) => {
            println!("cnx err : {}", e);
        }
    }
}

fn traite_client(mut stream: TcpStream) {
    //mut stream: TcpStream
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            data.reverse(); // let t = s.chars().rev().collect::<String>();
            stream.write(&data[50 - size..]).unwrap();
            print!("{}", from_utf8(&data[50 - size..]).unwrap());
            true
        }
        Err(_) => {
            println!("fin de cnx avec {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn server() {
    println!("Server");
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("alu {}:", stream.peer_addr().unwrap());
                //println!("client {}", client.local_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    traite_client(stream)
                });
            }
            Err(e) => {
                println!("cnx err : {}", e);
            }
        }
    }
}

fn client() {
    match TcpStream::connect("127.0.0.1:8888") {
        Ok(mut stream) => {
            println!("cnx ok {}", stream.peer_addr().unwrap());
            loop {
                let mut buf = [0u8; 6];
                stdin().read(&mut buf).unwrap();
                // let car = from_utf8(&buf).unwrap();
                stream.write(&buf).unwrap();
            }
        }
        Err(e) => {
            println!("cnx err : {}", e);
        }
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("mauvaise commande");
        return;
    }
    match args[1].as_str() {
        "client" => client(),
        "server0" => server0(),
        "server" => server(),
        _ => {
            println!("mauvaise commande")
        }
    }
}
