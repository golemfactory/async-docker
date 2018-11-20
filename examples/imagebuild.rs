extern crate async_docker;
extern crate futures;
extern crate http;
extern crate tokio;

use async_docker::{new_docker, BuildOptions, DockerApi};
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
        let opts = BuildOptions::builder(path).tag("async_docker_test").build();
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        docker
            .images()
            .build(&opts)
            .and_then(|a| Ok(println!("{:#?}", a)))
            .map_err(|e| eprintln!("{:?}", e))
    });

    tokio::runtime::run(work);
}
