extern crate shiplift;
extern crate http;
extern crate futures;
extern crate tokio;

use shiplift::{DockerApi, new_docker};
use http::Uri;
use std::env;
use futures::{future, Future};

fn main() {
    let uri : Uri = env::args().nth(1)
        .unwrap_or("unix://var/run/docker.sock".to_string())
        .parse().unwrap();

    let work = future::lazy(||  {
        let docker: Box<DockerApi> = new_docker(uri).unwrap();

        docker
            .info()
            .then(|a| Ok(println!("{:?}", a)))
    });

    tokio::runtime::run(work);
}