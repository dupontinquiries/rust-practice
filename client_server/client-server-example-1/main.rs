use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};

struct Message {
    sender: String,
    body: String,
}

struct Server {
    messages: Arc<Mutex<Vec<Message>>>,
    listener: TcpListener,
}

impl Server {
    fn new(messages: Arc<Mutex<Vec<Message>>>, listener: TcpListener) -> Server {
        Server { messages, listener }
    }

    fn accept_clients(&self) {
        for stream in self.listener.incoming() {
            let messages = self.messages.clone();
            thread::spawn(move || {
                let mut stream = stream.unwrap();
                loop {
                    let mut buf = String::new();
                    stream.read_to_string(&mut buf).unwrap();
                    let mut messages = messages.lock().unwrap();
                    messages.push(Message {
                        sender: "unknown".to_string(),
                        body: buf,
                    });
                }
            });
        }
    }
}

struct Client {
    messages: Arc<Mutex<Vec<Message>>>,
    stream: TcpStream,
}

impl Client {
    fn new(messages: Arc<Mutex<Vec<Message>>>, stream: TcpStream) -> Client {
        Client { messages, stream }
    }

    fn send_message(&mut self, message: &str) {
        self.stream.write(message.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    fn receive_messages(&mut self) {
        loop {
            let mut messages = self.messages.lock().unwrap();
            for message in messages.iter() {
                println!("{}: {}", message.sender, message.body);
            }
        }
    }
}

fn main() {
    let messages = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let server = Server::new(messages.clone(), listener);
    thread::spawn(move || server.accept_clients());

    let mut client = Client::new(messages.clone(), TcpStream::connect("127.0.0.1:8080").unwrap());
    client.send_message("Hello, world!");
    client.receive_messages();
}
