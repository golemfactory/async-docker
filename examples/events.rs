extern crate shiplift;
extern crate http;
extern crate futures;
extern crate tokio;

use shiplift::{DockerApi, new_docker, EventsOptionsBuilder};
use http::Uri;
use std::env;
use futures::{future, Stream, Future};

fn main() {
    let uri : Uri = env::args().nth(1)
        .unwrap_or("unix://var/run/docker.sock".to_string())
        .parse().unwrap();

    let work = future::lazy(||  {
        let opts = EventsOptionsBuilder::default().build();
        let docker: Box<DockerApi> = new_docker(uri).unwrap();

        docker
            .events(&opts)
            .for_each(|a| Ok(println!("{:?}", a)))
            .map_err(|e| eprintln!("{:?}", e))
    });

    tokio::runtime::run(work);
}