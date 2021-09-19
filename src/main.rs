use std::io::{stdin, stdout, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

// utiliser clap
const BUFFSIZE_CLIENT: usize = 6;
const BUFFSIZE_SERVER: usize = 50;

// un serveur qui teste si une connexion est possible
fn server0() {
    println!("Server");
    let listener = TcpListener::bind("0.0.0.0:8888").unwrap();
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
// traitement cote serveur qui retourne une chaine dans l ordre inverse
fn _traite_client(mut stream: TcpStream) {
    //mut stream: TcpStream
    let client_tcp = stream.peer_addr().unwrap().to_string();
    let mut data = [0 as u8; BUFFSIZE_SERVER];
    //println!("flush {}", client_tcp);
    //stream.flush().unwrap();
    println!("<...");
    while match stream.read(&mut data) {
        Ok(size) => {
            // traitement
            //data.reverse(); // let t = s.chars().rev().collect::<String>();
            let chaine = from_utf8(&data).unwrap();
            let chaine_rev: String = chaine.chars().rev().collect();
            println!("{} len {} > {}", client_tcp, size, chaine);
            //stream.write(&data[BUFFSIZE_SERVER - size - 1..]).unwrap();
            let data_to_send = chaine_rev.as_bytes();
            println!("arr {:?} chaine {:?}", data_to_send, chaine_rev);
            // fin traitement
            let data_utiles = &data_to_send[BUFFSIZE_SERVER - size..];
            println!(
                "envoi {} > {}",
                data_to_send.len(),
                from_utf8(&data_to_send).unwrap()
            );
            stream.write(data_utiles).unwrap();
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
fn traitement_serveur(
    data: &[u8; BUFFSIZE_SERVER],
    reponse: &mut [u8; BUFFSIZE_SERVER],
    player: &std::sync::Arc<std::sync::Mutex<Player>>,
) -> usize {
    {
        let mut player_partagee = player.lock().unwrap();
        player_partagee.x += 1;
    }
    reponse[0] = data[0];
    let zero = b"0"; //.as_bytes();
    reponse[1] = zero[0];
    2
}
struct MyString(String);

impl MyString {
    fn new(s: &str) -> MyString {
        MyString(s.to_string())
    }
}

struct Player {
    nom: String,
    x: i32,
    y: i32,
}

impl Player {
    fn new(s: &str) -> Player {
        Player {
            nom: s.to_string(),
            x: 0,
            y: 0,
        }
    }
    fn deplace0(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    fn deplace(p: &mut Player, x: i32, y: i32) {
        p.x = x;
        p.y = y;
    }
}

// struct Player<'a> {
//     stream: &'a std::net::TcpStream,
// }
//
// struct Player2 {
//     stream: std::net::TcpStream,
// }

use std::sync::{Arc, Mutex};

// TODO : a l'arret de node red, bloucle en envoyant des buffers vides
fn server() {
    println!("Server");
    let listener = TcpListener::bind("0.0.0.0:8888").unwrap();
    println!("...");
    let mut _players: [Option<Player>; 8] = [None, None, None, None, None, None, None, None];
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("alu {}:", stream.peer_addr().unwrap());
                //players[0] = Some(Player { stream: &stream });
                let _client_tcp = stream.peer_addr().unwrap().to_string();
                let p = Player::new("bob");
                let p_ark = Arc::new(Mutex::new(p));
                let player_clone = Arc::clone(&p_ark);
                thread::spawn(move || {
                    //let mut data_lu = [0 as u8; BUFFSIZE_SERVER];
                    let mut data = [0 as u8; BUFFSIZE_SERVER];
                    let mut reponse = [0 as u8; BUFFSIZE_SERVER];
                    println!("<...");
                    while match stream.read(&mut data) {
                        Ok(0) => {
                            println!("rien a lire");
                            false
                        }
                        Ok(BUFFSIZE_SERVER) => true, // on ne traite pas les trop gros paquets
                        Ok(_size) => {
                            let size_reponse =
                                traitement_serveur(&data, &mut reponse, &player_clone);
                            stream.write(&reponse[..size_reponse]).unwrap();
                            //stream.flush().unwrap();
                            println!("<...");
                            true
                        }
                        Err(_) => {
                            println!("fin de cnx avec {}", stream.peer_addr().unwrap());
                            stream.shutdown(Shutdown::Both).unwrap();
                            false
                        }
                    } {}
                });
            }
            Err(e) => {
                println!("cnx err : {}", e);
            }
        }
    }
}

fn _client_lecture0(stream: &mut TcpStream) {
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

fn _client_lecture1(stream: &mut TcpStream) {
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
        Ok(0) => {
            println!("rien lu");
            false
        }
        Ok(BUFFSIZE_CLIENT) => {
            println!("un paquet server > {}", from_utf8(&data).unwrap());
            true
        }
        Ok(size) => {
            println!("lu stream {} server > {}", size, from_utf8(&data).unwrap());
            false
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
                stdout().flush().unwrap(); // parcequ pas println
                let lu_stdin = stdin().read(&mut wbuf).unwrap();
                //println!();
                println!("lu stdin {}", lu_stdin);
                // let car = from_utf8(&wbuf).unwrap();
                match stream.write(&wbuf[..lu_stdin - 1]) {
                    // eventuellement write_all plus tard
                    Ok(0) => {
                        println!("ecriture buffer vide ?");
                    }
                    Ok(size) => {
                        println!("ecriture {}", size);
                        println!("<...");
                        client_lecture(&mut stream);
                    }
                    Err(e) => {
                        println!("cnx terminee : {}", e);
                        stream.shutdown(Shutdown::Both).unwrap();
                    }
                }
                // stream.flush().unwrap();
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
