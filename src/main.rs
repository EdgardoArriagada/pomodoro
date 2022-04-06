use std::io::{BufRead, BufReader};
use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::str::FromStr;

struct Args {
    duration: String,
    message: String,
}

static DEFAULT_MESSAGE: &'static str = "No Message";

impl FromStr for Args {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<&str> = s.split(' ').collect();
        
        match vec.len() {
            0 => Err(()),
            1 => {
                let duration = vec[0].to_string();
                let message = DEFAULT_MESSAGE.to_string();
                Ok(Args { duration, message })
            },
            _ => {
                let duration = vec[0].to_string();
                let message = vec[1..vec.len()].join(" ");
                Ok(Args { duration, message })
            },
        }
    }
}

fn handle_client(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        let Args {duration, message} = Args::from_str(&line.unwrap()).unwrap();
        println!("duration: {}, message: {}", duration, message);
    }
}

fn main() -> std::io::Result<()> {
    let listener = UnixListener::bind("/tmp/rust-uds.sock").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
    Ok(())
}
