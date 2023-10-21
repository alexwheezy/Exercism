use std::{sync::Arc, thread};

fn foo(input: &[&str]) {
    //let input = input.iter().map(|&s| s.to_owned()).collect::<Vec<String>>();
    //    thread::spawn(move || println!("{:?}", &input))
    //        .join()
    //        .unwrap();
}

fn main() {
    const FORMATS: [&str; 3] = ["hip", "hiplc", "hipnc"];
    foo(&FORMATS);
}
