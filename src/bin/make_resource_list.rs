extern crate hyper;
extern crate html5ever;
#[macro_use] extern crate string_cache;

use std::fs::File;
use std::io::prelude::*;

use hyper::Client;
use html5ever::tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Element, RcDom, Handle};

use std::default::Default;
use std::string::String;


fn walk(indent: usize, handle: Handle, mut resource_list: &mut Vec<String>)  {
    let node = handle.borrow();
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

fn make_resource_list(url: &str, client: &Client) {
    let mut response = client.get(url).send().expect("Couldn't get response");

    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Error reading response");

    let dom = parse_document(RcDom::default(), Default::default()).one(buf);
    let mut resource_list = vec!();
    walk(0, dom.document, &mut resource_list);
  
    let mut writer = File::create("resources.txt").unwrap();
  
    for r in resource_list{
        if !r.starts_with("http") {
            write!(writer, "{}{}\n", url, r).expect("IO Error");
        } else {
            write!(writer, "{}\n", url).expect("IO Error");
        }
    }
}

fn main() {
    let client = Client::new();
    let url = "https://abbyputinski.com";

    make_resource_list(&url, &client);
}


