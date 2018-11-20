extern crate async_docker;
extern crate futures;
extern crate http;
extern crate tokio;

use async_docker::LogsOptionsBuilder;
use async_docker::{new_docker, DockerApi};
use futures::Stream;
use futures::{future, Future};
use std::env;

fn main() {
    if env::args().count() < 2 {
        println!("Too few arguments (<1).");
        return;
    }

    let container = env::args().nth(1).unwrap();

    let work = future::lazy(move || {
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        let opts = LogsOptionsBuilder::new().stdout(true).build();

        docker
            .container(container.into())
            .logs(&opts)
            .for_each(|a| Ok(println!("{:?}", a)))
            .map_err(|e| eprintln!("{:?}", e))
    });

    tokio::runtime::run(work);
}
