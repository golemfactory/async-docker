extern crate async_docker;
extern crate futures;
extern crate http;
extern crate tokio;

use async_docker::{new_docker, DockerApi};
use futures::{future, Future, Stream};
use std::env;

fn main() {
    let id = match env::args().nth(1) {
        Some(val) => val,
        None => {
            println!("Not enough arguments");
            return;
        }
    };

    let work = future::lazy(move || {
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        docker
            .container(id.into())
            .stats()
            .for_each(|a| Ok(println!("{:?}", a)))
            .map_err(|e| eprintln!("{:?}", e))
    });

    tokio::runtime::run(work);
}
