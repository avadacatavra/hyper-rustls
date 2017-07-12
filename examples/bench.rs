extern crate futures;
extern crate hyper;
extern crate hyper_rustls;
extern crate rustls;
extern crate tokio_core;

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::time::{Duration, Instant};
use std::vec::Vec;

use hyper::{Client, Uri};
use std::str::FromStr;

use hyper_rustls::HttpsConnector;
use hyper::StatusCode;

use tokio_core::reactor::Core;

fn duration_nanos(d: Duration) -> f64 {
    (d.as_secs() as f64) + (d.subsec_nanos() as f64) / 1e9
}

// uses default configuration
fn create_client(core: &Core) -> Client<HttpsConnector> {
	Client::configure()
         .connector(HttpsConnector::new(4, &core.handle()))
         .build(&core.handle())
}

// TODO use #[bench]?

pub fn website_bench(site: &str) -> f64 {

	let mut core = tokio_core::reactor::Core::new().unwrap();
	let client = create_client(&core);
    let start = Instant::now();
    let res = core.run(client.get(Uri::from_str(site).unwrap())).unwrap();
    assert!(res.status().is_success() || res.status() == StatusCode::Found);

    duration_nanos(Instant::now().duration_since(start))

}


fn main() {

	let mut file = match File::open(Path::new("./examples/sites.txt")) {
        Err(_) => panic!("sites.txt not found"),
        Ok(file) => file,
    };
    // can create a custom root-ca store (defaults to webpki)

    let mut sites = String::new();
    file.read_to_string(&mut sites).unwrap();

    

    let mut times: Vec<f64> = vec!();
    for line in sites.lines() {
    	//TODO fix sites.txt
        let l: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
        let mut site = "https://".to_owned();
        site.push_str(l[0].trim());

        let mut site_time: Vec<f64> = vec!();
        for _ in 0..10 {
        	site_time.push(
        		website_bench(&site));
        }
        let avg = site_time.iter().fold(0.0, |a, &b| a + b)/(site_time.len() as f64);
        times.push(avg);
    }

    println!("Average times for connection (ns)");
	println!("{:?}", times)
}
