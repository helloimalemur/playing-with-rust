
use std::io::Read;
use std::process::{Command, Stdio};

pub fn h(string:&str) {
    println!("{}", string);
}

pub fn test_thread() {

    let process = match Command::new("ping")
        .args(vec!["-c 1", "10.0.0.1"])
        .stdout(Stdio::piped())
        .spawn() {
        Err(why) => panic!("{}", why),
        Ok(process) => process
    };



    let mut s = String::new();

    match process.stdout.unwrap().read_to_string(&mut s) {
        Ok(n) => println!("{}", s),
        Err(why) => println!("{}", why)
    }
}
