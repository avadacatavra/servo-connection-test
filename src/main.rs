extern crate hyper;

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
use hyper::{Client};


//http://zsiciarz.github.io/24daysofrust/book/day5.html
fn main() {

    let client = Client::new();
    let url = "http://i.imgur.com/PwEwUhA.jpg";
    //let url = "http://zsiciarz.github.io/24daysofrust/book/day5.html";
    

    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => panic!("Error"),
    };


    let path = Path::new("out/test.out");
    let display = path.display();
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", 
                           display, why.description()),
        Ok(file) => file,
    };
    let mut writer = BufWriter::new(file);

    //let mut buf = String::new();
    let mut buf = Vec::new();
    match response.read_to_end(&mut buf){
        Ok(_) => match writer.write(buf.as_slice()) {
            Err(why) => panic!("couldn't write to {}: {}", 
                               display, why.description()),
            Ok(_) => println!("successfully wrote to {}", display)
        },
        Err(why) => panic!("couldn't read response: {}", why.description())
    }



}
