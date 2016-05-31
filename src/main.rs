extern crate hyper;
extern crate html5ever;
extern crate scoped_threadpool;
#[macro_use] extern crate string_cache;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;
use std::path::Path;

use hyper::Client;
use hyper::header::Connection;
use html5ever::tendril::TendrilSink;
use html5ever::parse_document;
use html5ever::rcdom::{Element, RcDom, Handle};

use std::default::Default;
use std::string::String;

use scoped_threadpool::Pool;
use std::sync::Arc;

//TODO switch to rust-url https://github.com/servo/rust-url

fn get_filename_from_url(url : &str) -> String {
    url.replace("/", "_")
}


fn fetch_resource(url: &str, client: &Client){
    let mut response =  client.get(url)
        //.header(Connection::close())
        .send()
        .expect("Error getting url");

    write_resource(url, &mut response); 
}

fn write_resource(url: &str, response: &mut hyper::client::Response){ //FIXME
    let filename = format!("./out/{}",get_filename_from_url(&url));
    let path = Path::new(&filename);
    let file =  File::create(&path).unwrap();
    let mut writer = BufWriter::new(file);

    let mut buf = Vec::new();
    if response.read_to_end(&mut buf).is_ok() {
        writer.write(buf.as_slice()).expect("IO Error");
    }
}

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


//goal: model resource fetching to examine hyper connection behavior
fn main() {
    let client = Arc::new(Client::new());

    let url = "https://abbyputinski.com";
    //let url = "https://twitter.com";

    /*let output_dir = fs::metadata("./out");
    if output_dir.is_err() || !output_dir.unwrap().is_dir(){    //FIXME
        fs::create_dir("./out").expect("Couldn't create ./out");
    }*/

    if !Path::new("./out").is_dir() {
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
 

    /*
     *  TODO not sure if this is the best way 
     *  http://seanmonstar.com/post/141495445652/async-hyper
     */
    let mut pool = Pool::new(8);
    pool.scoped(|scope| {
        for l in resources.lines() {
            let c = client.clone();
            scope.execute(move || {
                let line = l.unwrap();
                fetch_resource(&line, &c);
            });
        }
    });

    /*for l in resources.lines() {

        let line = l.unwrap();
        fetch_resource(&line, &client);
    }
    */
}

//TODO https://github.com/kbknapp/clap-rs for cmd line
////TODO make make_resources into srs/bin/make_resources.rs
//// <jack> note once there are multiple binaries, you have to use --bin to cargo run to pick which one you want
