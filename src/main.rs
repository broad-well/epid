
use epid::*;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        let read = stdin.read_line(&mut line);
        if read.is_err() {
            break
        }
        let tokens = line.trim().split(' ').collect::<Vec<_>>();

        match tokens[0] {
            "ip" => println!("{}", ipv4::ipv4_to_epid3(tokens[1]).unwrap_or("<bad IP>".into())),
            "epid" => println!("{}", ipv4::epid3_to_ipv4(tokens[1]).unwrap_or("<bad EPID3>".into())),
            "quit" => break,
            _ => println!("Unknown command. Try ip or epid.")
        }
        
        line.clear();
    }
}