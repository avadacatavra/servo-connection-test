extern crate hyper;
extern crate html5ever;
extern crate clap;
extern crate url;
#[macro_use] extern crate string_cache;

use std::fs::File;
use std::io::prelude::*;

use hyper::Client;
use html5ever::tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Element, RcDom, Handle};
use url::{Url, ParseError};

use std::default::Default;
use std::string::String;

use clap::{Arg,App};


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
    let base_url = Url::parse(url).unwrap();
    assert!(!base_url.cannot_be_a_base());

    let mut response = client.get(url).send().expect("Couldn't get response");

    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Error reading response");

    let dom = parse_document(RcDom::default(), Default::default()).one(buf);
    let mut resource_list = vec!();
    walk(0, dom.document, &mut resource_list);
  
    let mut writer = File::create("resources.txt").unwrap();
    
    write!(writer, "{}\n", url);
    for resource_url in resource_list{
        if Url::parse(&resource_url) == Err(ParseError::RelativeUrlWithoutBase) {
            let resource_url = base_url.join(&resource_url).unwrap();
            write!(writer, "{}\n", resource_url).unwrap();
            
        } else{
            write!(writer, "{}\n", resource_url).unwrap();  //FIXME duplicative
        }
    }
}



fn main() {
    let matches = App::new("make-resource-list")
                            .bin_name("make_resource_list")
                            .version("1.0")
                            .author("Diane Hosfelt dhosfelt@mozilla.com")
                            .about("Makes a list of resources from a URL")
                            .arg(Arg::with_name("URL")
                                .short("u")
                                .long("url")
                                .help("the url to grab resources from")
                                .takes_value(true))
                            .get_matches();
    let url = matches.value_of("URL").unwrap_or("https://abbyputinski.com");

    let client = Client::new();

    make_resource_list(&url, &client);
}

