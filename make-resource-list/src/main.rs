extern crate hyper;
extern crate html5ever;
//extern crate getopts;
#[macro_use] extern crate string_cache;

//TODO clean up unused stuff
//TODO command line args? library? something so i don't provide a url like this?

use hyper::{Client};
use std::io::prelude::*;
use std::error::Error;
use std::string::String;

use std::default::Default;

use html5ever::tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};

use std::fs::File;

//from https://github.com/servo/html5ever/blob/master/examples/print-rcdom.rs
fn walk(indent: usize, handle: Handle, mut resource_list : &mut Vec<String>)  {

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
                    //resource_list.push(replace(&mut attr.value.to_string(), String::new()));
                    resource_list.push(attr.value.to_string());
                }
            }
        }
    }

    for child in node.children.iter() {
        walk(indent+4, child.clone(), &mut resource_list);
    }


}


fn main() {
    let client = Client::new();
    let url = "https://abbyputinski.com";

    //get the page
    let mut response = match client.get(url).send(){
        Err(why) => panic!("Couldn't get response: {}", why.description()),
        Ok(response) => response,
    };

    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Err(why) => panic!("Couldn't read response: {}", why.description()),
        Ok(_) => (),
    };

    
    //grab the list of resources
    let dom = parse_document(RcDom::default(), Default::default()).one(buf);
    let mut resource_list = vec!();
    walk(0, dom.document, &mut resource_list);
   
    /*println!("{}", resource_list.len());
    for i in resource_list.iter(){
        println!("{}", i);
    }*/
  
    //TODO output resource_list to file
    let mut writer = match File::create("../resources.txt") {
        Err(why) => panic!("Can't create resources.txt: {}", why.description()),
        Ok(writer) => writer,
    };

    for r in resource_list.iter(){
        write!(writer, "{}\n", r).expect("IO Error");
    }


}
