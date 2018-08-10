extern crate shiplift;
extern crate http;
extern crate futures;
extern crate tokio;

use shiplift::{DockerApi, new_docker};
use std::env;
use futures::{future, Future, Stream};

fn main() {
    let id = match env::args().nth(1) {
        Some(val) => val,
        None => {
            println!("Not enough arguments");
            return;
        }
    };

    let work = future::lazy(move||  {
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        docker
            .container(id.into())
            .stats()
            .for_each(|a| Ok(println!("{:?}", a)))
            .map_err(|e| eprintln!("{:?}", e))
    });

    tokio::runtime::run(work);
}