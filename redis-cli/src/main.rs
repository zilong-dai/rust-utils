use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;

fn main() {
    let mut args = std::env::args();
    args.next();

    if let Some(address) = args.next() {
        if let Ok(mut stream) = TcpStream::connect(&address) {
            println!("Connected to Redis Server: {}", address);

            let mut stdout = std::io::stdout();

            loop {
                print!("redis> ");
                stdout.flush().unwrap();

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                if input.trim() == "quit" || input.trim() == "exit" || input.trim() == "q" {
                    break;
                }

                stream.write(input.as_bytes()).unwrap();

                // if return an array, first is the number of elements in the array like "*4"
                // *0 indicate empty array
                // $keylen \r\n value \r\n
                // keylen == -1 indicate the key is not exists
                // todo: read all the data in a loop
                let mut response = [0u8; 1024];
                let _len = stream.read(&mut response).unwrap();

                let response_str = String::from_utf8_lossy(&response[0.._len]).into_owned();

                let split_response = response_str.trim().split("\r\n").collect::<Vec<&str>>();
                println!("{:?}", split_response);
            }
        } else {
            eprintln!("Error connecting to Redis: {}", address);
            exit(1);
        }
    } else {
        eprintln!("Please provide the Redis server address.");
        exit(1);
    }
}
