extern crate shiplift;
extern crate http;
extern crate futures;
extern crate tokio;

use shiplift::{DockerApi, new_docker, BuildOptions};
use futures::{future, Future};
use std::env;

fn main() {
    let path = match env::args().nth(1) {
        Some(val) => val,
        None => {
            println!("Not enough arguments");
            return;
        }
    };

    let work = future::lazy(|| {
        let opts = BuildOptions::builder(path).tag("shiplift_test").build();
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        docker
            .images()
            .build(&opts)
            .and_then(|a| Ok(println!("{:#?}", a)))
            .map_err(|e| eprintln!("{:?}", e))
    });

    tokio::runtime::run(work);
}