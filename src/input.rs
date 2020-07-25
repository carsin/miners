use std::io::{stdin, Read};
use std::sync::mpsc::{channel, Sender, Receiver};

pub fn get_input() -> Receiver<char> {
    let mut stdin = stdin();
    let (input_sender, input_receiver): (Sender<char>, Receiver<char>) = channel();
    std::thread::spawn(move || {
        loop {
            let mut buf = [0u8; 1];
            stdin.read_exact(&mut buf).unwrap();
            input_sender.send(buf[0] as char).unwrap();
        }
    });

    input_receiver
}
