extern crate async_docker;
extern crate futures;
extern crate http;
extern crate tokio;

use async_docker::{new_docker, DockerApi};
use futures::{future, Future};

fn main() {
    let work = future::lazy(|| {
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        docker.info().then(|a| Ok(println!("{:?}", a)))
    });

    tokio::runtime::run(work);
}
