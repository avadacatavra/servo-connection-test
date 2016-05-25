extern crate hyper;
extern crate html5ever;

use hyper::{Client};
use std::io::{self, Read};
use std::error::Error;
use std::string::String;

use html5ever::tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};

fn main() {
    let client = Client::new();

    let url = "https://abbyputinski.com";

    let mut response = match client.get(url).send(){
        Err(why) => panic!("Couldn't get response: {}", why.description()),
        Ok(response) => response,
    };

    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Err(why) => panic!("Couldn't read response: {}", why.description()),
        Ok(_) => (),
    };

    //println!("{}",buf)
    
    //So buf holds the html of the site. Now you need to parse through it and grab 

    let stdin = io::stdin();
    let dom = parse_document(RcDom::default(), Default::default()).one(buf);
   

    let handle = dom.document;
    let node = handle.borrow();
    match node.node {
        Document => (),
        Doctype(_) => (),
        Text(ref text) => (),
        Element(ref name, _,ref attrs) => {
           // assert!(name.ns == ns!(html));
            print!("<{}", name.local);
        }
    }


}
