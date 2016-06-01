extern crate hyper;
extern crate html5ever;
extern crate scoped_threadpool;
extern crate clap;
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
use clap::{Arg,App};

//TODO is there a way to do this with rust-url?
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


fn main() {
    let client = Arc::new(Client::new());

    //TODO which way of making this is preferred? https://github.com/kbknapp/clap-rs
    let matches = App::new("servo-connection-test")
                            .bin_name("servo_connection_test")
                            .version("1.0")
                            .author("Diane Hosfelt dhosfelt@mozilla.com")
                            .about("models resource fetching with hyper")
                            .arg(Arg::with_name("threads")
                                .short("t")
                                .long("threads")
                                .help("number of threads in connection pool")
                                .takes_value(true))
                            .get_matches();
    let threads = matches.value_of("threads").unwrap_or("8").parse::<u32>().unwrap();

    if !Path::new("./out").is_dir() {
        fs::create_dir("./out").expect("Couldn't create ./out");
    }


    //open resources.txt and iterate through lines
    let path = Path::new("resources.txt");
    let exists = std::fs::metadata(path);
    if exists.is_err() {
       panic!("Please create resources.txt"); 
    }
    let file = File::open(&path).unwrap();
    let mut resources = BufReader::new(file);
   
    //TODO not sure if i actually need this
    let mut base_url = String::new();
    resources.read_line(&mut base_url).unwrap();
    println!("{}", base_url);

    /*
     *  TODO not sure if this is the best way 
     *  http://seanmonstar.com/post/141495445652/async-hyper
     */
    let mut pool = Pool::new(threads);
    pool.scoped(|scope| {
        for l in resources.lines() {
            let c = client.clone();
            scope.execute(move || {
                let line = l.unwrap();
                fetch_resource(&line, &c);
            });
        }
    });
}

////TODO benchmarking https://doc.rust-lang.org/book/benchmark-tests.html
