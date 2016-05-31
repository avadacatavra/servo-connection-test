extern crate hyper;
extern crate html5ever;
#[macro_use] extern crate string_cache;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;
use std::path::Path;

use hyper::{Client};
use html5ever::tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Element, RcDom, Handle};

use std::default::Default;
use std::string::String;


fn get_filename_from_url(url : &str) -> String {
    url.replace("/", "_")
}


//fetch resource and write to file
fn fetch_resource(url : &str, client : &Client){
    let mut response =  client.get(url).send().expect("Error getting url");

   
    let filename = format!("./out/{}",get_filename_from_url(&url));
    println!("{}",filename);
    let path = Path::new(&filename);
    let file =  File::create(&path).unwrap();
    let mut writer = BufWriter::new(file);

    let mut buf = Vec::new();
    if response.read_to_end(&mut buf).is_ok() {
        writer.write(buf.as_slice()).expect("IO Error");
    }


}

fn walk(indent: usize, handle: Handle, mut resource_list : &mut Vec<String>)  {

    let node = handle.borrow();
    // FIXME: don't allocate
    // FIXME: do I really need all of those unused match compares?
    match node.node {
        Element(ref name, _, ref attrs) => {
            assert!(name.ns == ns!(html));
            for attr in attrs.iter() {
                assert!(attr.name.ns == ns!());
                if attr.name.local.eq_ignore_ascii_case(&atom!("src")) || 
                   attr.name.local.eq_ignore_ascii_case(&atom!("href")) {
                    resource_list.push(attr.value.to_string());
                }
            }
        }
        _ => (),
    }

    for child in node.children.iter() {
        walk(indent+4, child.clone(), &mut resource_list);
    }


}

//FIXME duplication with fetch_resource
fn make_resource_list(url : &str, client : &Client) {
    let mut response = client.get(url).send().expect("Couldn't get response");

    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Error reading response");

    let dom = parse_document(RcDom::default(), Default::default()).one(buf);
    let mut resource_list = vec!();
    walk(0, dom.document, &mut resource_list);
  
    let mut writer = File::create("resources.txt").unwrap();
  
    for r in resource_list.iter(){
        if !r.starts_with("http") {
            write!(writer, "{}{}\n", url, r).expect("IO Error");
        } else {
            write!(writer, "{}\n", url).expect("IO Error");
        }
    }

}



//http://zsiciarz.github.io/24daysofrust/book/day5.html
fn main() {

    let client = Client::new();
    let url = "https://abbyputinski.com";
   
    let output_dir = fs::metadata("./out");
    if output_dir.is_err() || !output_dir.unwrap().is_dir(){
        fs::create_dir("./out").expect("Couldn't create ./out");
    }


    //open resources.txt and iterate through lines
    let path = Path::new("resources.txt");

    let exists = std::fs::metadata(path);
    if exists.is_err() {
        make_resource_list(&url, &client);
    }

    
    let file = File::open(&path).unwrap();


    let resources = BufReader::new(file);
    
    for l in resources.lines() {
        let line = l.unwrap();
        println!("{}", line);
        fetch_resource(&line, &client);
    }

}
