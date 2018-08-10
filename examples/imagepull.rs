extern crate shiplift;
extern crate http;
extern crate futures;
extern crate tokio;

use shiplift::{DockerApi, new_docker, PullOptions};
use futures::{future, Future};
use std::env;

fn main() {
    let image = match env::args().nth(1) {
        Some(val) => val,
        None => {
            println!("Not enough arguments");
            return;
        }
    };

    let work = future::lazy(|| {
        let opts = PullOptions::builder().image(image).build();
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        docker
            .images()
            .pull(&opts)
            .and_then(|a| Ok(println!("{:#?}", a)))
            .map_err(|e| eprintln!("{:?}", e))
    });

    tokio::runtime::run(work);
}