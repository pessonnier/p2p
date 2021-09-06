use std::io::{stdin, stdout, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

// utiliser clap
const BUFFSIZE_CLIENT: usize = 6;
const BUFFSIZE_SERVER: usize = 50;

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
    let client_tcp = stream.peer_addr().unwrap().to_string();
    let mut data = [0 as u8; BUFFSIZE_SERVER]; // using 50 byte buffer
    println!("flush {}", client_tcp);
    stream.flush().unwrap();
    println!("<...");
    while match stream.read(&mut data) {
        Ok(size) => {
            data.reverse(); // let t = s.chars().rev().collect::<String>();
            println!("{} > {}", client_tcp, from_utf8(&data).unwrap());
            stream.write(&data[BUFFSIZE_SERVER - size - 1..]).unwrap();
            stream.flush().unwrap();
            println!("<...");
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

fn client_lecture0(stream: &mut TcpStream) {
    let mut rbuf = [0u8; BUFFSIZE_CLIENT]; //Vec<u8> = Vec::new();
    loop {
        let lu_stream = stream.read(&mut rbuf).unwrap();
        if lu_stream == 0 {
            break;
        }
        println!("lu stream {}", lu_stream);
        println!("server > {}", from_utf8(&rbuf).unwrap());
    }
}

fn client_lecture1(stream: &mut TcpStream) {
    let mut rbuf: Vec<u8> = Vec::new();
    let lu_stream = stream.read_to_end(&mut rbuf).unwrap();
    if lu_stream == 0 {
        println!("pas de message");
    }
    println!("lu stream {}", lu_stream);
    println!("server > {}", from_utf8(&rbuf).unwrap());
}

fn client_lecture(stream: &mut TcpStream) {
    let mut data = [0u8; BUFFSIZE_CLIENT];
    while match stream.read(&mut data) {
        Ok(0) => false,
        Ok(size) => {
            println!("lu stream {}", size);
            println!("server > {}", from_utf8(&data).unwrap());
            true
        }
        Err(e) => {
            println!("cnx err : {}", e);
            false
        }
    } {}
}

fn client() {
    match TcpStream::connect("127.0.0.1:8888") {
        Ok(mut stream) => {
            println!("cnx ok {}", stream.peer_addr().unwrap());
            stream.flush().unwrap();
            //stream.set_nodelay(true);
            loop {
                let mut wbuf = [0u8; BUFFSIZE_CLIENT];
                print!("> ");
                stdout().flush().unwrap();
                let lu_stdin = stdin().read(&mut wbuf).unwrap();
                //println!();
                println!("lu stdin {}", lu_stdin);
                // let car = from_utf8(&wbuf).unwrap();
                stream.write_all(&wbuf).unwrap();
                stream.flush().unwrap();
                println!("<...");
                client_lecture(&mut stream);
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
