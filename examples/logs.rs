extern crate shiplift;
extern crate http;
extern crate futures;
extern crate tokio;

use std::env;
use shiplift::{DockerApi, new_docker};
use futures::{future, Future};
use shiplift::LogsOptionsBuilder;
use futures::Stream;

fn main() {
    if env::args().count() < 2 {
        println!("Too few arguments (<1).");
        return;
    }

    let container = env::args().nth(1).unwrap();

    let work = future::lazy(move ||  {
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        let opts = LogsOptionsBuilder::new().stdout(true).build();

        docker
            .container(container.into())
            .logs(&opts)
            .for_each(|a| Ok(println!("{:?}", a)))
            .map_err(|e|eprintln!("{:?}", e))
    });

    tokio::runtime::run(work);
}