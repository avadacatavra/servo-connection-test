extern crate hyper;
extern crate html5ever;
#[macro_use] extern crate string_cache;

use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;
use std::path::Path;

use hyper::{Client};
use html5ever::tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};

use std::default::Default;
use std::string::String;


//TODO create ./out if dne
//
fn get_filename_from_url(url : &str) -> String {
    url.replace("/", "_")
}


//fetch resource and write to file
fn fetch_resource(url : &str, client : &Client){
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => panic!("Error"),
    };

    //filename == replace all / with _
   
    let filename = get_filename_from_url(&url);
    let path = Path::new(&filename);
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


fn make_resource_list(url : &str, client : &Client) {
    let mut response = match client.get(url).send(){
        Err(why) => panic!("Couldn't get response: {}", why.description()),
        Ok(response) => response,
    };

    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Err(why) => panic!("Couldn't read response: {}", why.description()),
        Ok(_) => (),
    };

    let dom = parse_document(RcDom::default(), Default::default()).one(buf);
    let mut resource_list = vec!();
    walk(0, dom.document, &mut resource_list);
   
  
    let mut writer = match File::create("../resources.txt") {
        Err(why) => panic!("Can't create resources.txt: {}", why.description()),
        Ok(writer) => writer,
    };

    for r in resource_list.iter(){
        write!(writer, "{}\n", r).expect("IO Error");
    }

}



//http://zsiciarz.github.io/24daysofrust/book/day5.html
fn main() {

    let client = Client::new();
    //let url = "http://i.imgur.com/PwEwUhA.jpg";
    //let url = "http://zsiciarz.github.io/24daysofrust/book/day5.html";
    let url = "https://abbyputinski.com";
    

    fetch_resource(&url, &client);

    //open resources.txt and iterate through lines
    let path = Path::new("resources.txt");
    let file = match File::open(&path) {
        Err(_) => panic!("sigh"), //TODO on error, should form file
        Ok(file) => file,
    };
    let resources = BufReader::new(file);
    
    for line in resources.lines() {
        println!("{}", line.unwrap());
    }

}
