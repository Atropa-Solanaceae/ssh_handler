use std::io::{ self, BufRead };
use std::process::{ Command, Stdio };

fn main() {
    let mut child = Command::new("ssh")
        .arg("user@host")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start SSH connection");

    let stdout = child.stdout.as_mut().unwrap();
    let stdout_reader = io::BufReader::new(stdout);

    for line in stdout_reader.lines() {
        let line = line.unwrap();
        println!("[ssh] {}", line);
    }

    let status = child.wait().expect("Failed to wait on SSH process");
    println!("SSH process exited with status {:?}", status);
}