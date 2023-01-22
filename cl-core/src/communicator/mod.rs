use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;

use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn start_process(sender: Sender<String>, receiver: Receiver<String>, location: &str) {
    let child = Command::new("swipl")
        .arg(location)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    thread::spawn(move || {
        let mut f = BufReader::new(child.stdout.unwrap());
        let mut stdin = child.stdin.unwrap();
        for line in receiver {
            stdin.write_all(line.as_bytes()).unwrap();
            let mut buf = String::new();
            match f.read_line(&mut buf) {
                Ok(_) => {
                    sender.send(buf).unwrap();
                    continue;
                }
                Err(e) => {
                    println!("Error reading line: {:?}", e);
                    break;
                }
            }
        }
    });
}

fn start_command_thread(mutex: Mutex<Sender<String>>, command: String) {
    thread::spawn(move || {
        let sender = mutex.lock().unwrap();
        sleep(Duration::from_secs(1));
        sender
            .send(command)
            .unwrap();
    });
}

pub fn start_prolog(location: &str, command: &str) {
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    start_process(tx1, rx2, location);

    println!("started swipl");

    tx2.send(format!("one(a).\n")).unwrap();
    //start_command_thread(Mutex::new(tx2), command.to_string());

    println!("started thread");

    println!("{:?}", Vec::from_iter(rx1))
}
