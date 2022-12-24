use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::io::{Read, Write};

use git2::{Repository, RemoteCallbacks};
use git2::build::RepoBuilder;

struct Server {
    repo: Arc<Repository>,
}

impl Server {
    fn new(path: &Path) -> Server {
        let repo = Arc::new(Repository::open(path).unwrap());
        Server { repo }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let repo = self.repo.clone();
        thread::spawn(move || {
            loop {
                let mut buf = [0; 1024];
                match stream.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let request = String::from_utf8_lossy(&buf[..n]);
                        let response = self.handle_request(&request);
                        stream.write_all(response.as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                    Err(e) => panic!("{}", e),
                }
            }
        });
    }

    fn handle_request(&self, request: &str) -> String {
        let parts: Vec<&str> = request.split(' ').collect();
        let command = parts[0];
        let args = &parts[1..];
        match command {
            "push" => self.handle_push(args),
            "pull" => self.handle_pull(args),
            _ => format!("unknown command: {}", command),
        }
    }

    fn handle_push(&self, args: &[&str]) -> String {
        if args.len() != 1 {
            return String::from("usage: push <remote>");
        }
        let remote = args[0];
        let mut remote = self.repo.find_remote(remote).unwrap();
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_, _, _| Ok(git2::Cred::default
}