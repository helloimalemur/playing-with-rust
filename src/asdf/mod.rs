use std::process::Command;

pub fn h(string:&str) {
    println!("{}", string);
}

pub fn test_thread() {
    let _ = Command::new("firefox").spawn();
}
