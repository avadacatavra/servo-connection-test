extern crate hyper;
extern crate html5ever;
#[macro_use] extern crate string_cache;

use hyper::{Client};
use std::io::{self, Read};
use std::error::Error;
use std::string::String;

use std::iter::repeat;
use std::default::Default;
use std::mem::replace;

use html5ever::tendril::TendrilSink;
use html5ever::tendril::StrTendril;
use html5ever::parse_document;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};

//from https://github.com/servo/html5ever/blob/master/examples/print-rcdom.rs
//TODO missing lifetime specifier on Vec &str
fn walk(indent: usize, handle: Handle) -> Vec<String> {
    let mut resource_list = Vec::new();

    let node = handle.borrow();
    // FIXME: don't allocate
    // FIXME: do I really need all of those unused match compares?
    match node.node {
        Document => (),

        Doctype(_,_,_) => (),

        Text(_) => (),

        Comment(_) => (),

        Element(ref name, _, ref attrs) => {
            assert!(name.ns == ns!(html));
            for attr in attrs.iter() {
                assert!(attr.name.ns == ns!());
                //print!(" {}=\"{}\"", attr.name.local, attr.value);
                if attr.name.local.eq_ignore_ascii_case(&atom!("src")) || 
                   attr.name.local.eq_ignore_ascii_case(&atom!("href")) {
                    resource_list.push(replace(&mut attr.value.to_string(), String::new()))
                }
            }
        }
    }

    for child in node.children.iter() {
        walk(indent+4, child.clone());
    }

    for i in resource_list.iter(){
        println!("{}", i);
    }

    //TODO it's not returning?
    resource_list
}



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

    let dom = parse_document(RcDom::default(), Default::default()).one(buf);
    let resource_list = walk(0, dom.document);
   
    println!("{}", resource_list.len());
    for i in resource_list.iter(){
        println!("{}", i);
    }
    


}
