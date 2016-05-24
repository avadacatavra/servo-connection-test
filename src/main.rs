#![feature(core_intrinsics)]
extern crate hyper;

use std::io::Read;
use hyper::{Client};
//use net_traits::request::{RedirectMode, Referer, Request, RequestMode, ResponseTainting};


fn print_type_of<T>(_: &T) -> () {
    let type_name =
        unsafe {
            std::intrinsics::type_name::<T>()
        };
    println!("{}", type_name);
}

/*fn get_resource(url: &str) -> hyper::Result<String> {
       
}*/

//http://zsiciarz.github.io/24daysofrust/book/day5.html
fn main() {

    let client = Client::new();
    let url = "http://i.imgur.com/PwEwUhA.jpg";
    //let url = "http://zsiciarz.github.io/24daysofrust/book/day5.html";


    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => panic!("Error"),
    };

    print_type_of(&response);

    let mut buf = String::new();
    let mut buf2 : Vec<u8>;         //TODO hopefully I don't need 2 bufs...
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => match response.read_to_end(&mut buf2){
            Ok(_) => (),
            Err(_) => panic!("I give up."),
        }
    };
    //println!("buf: {}", buf);


}
