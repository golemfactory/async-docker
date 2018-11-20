extern crate async_docker;
extern crate futures;
extern crate http;
extern crate tokio;

use async_docker::{new_docker, DockerApi};
use futures::{future, Future};

fn main() {
    let work = future::lazy(|| {
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        docker
            .networks()
            .list(&Default::default())
            .and_then(|a| Ok(println!("{:?}", a)))
            .map_err(|e| eprintln!("{:?}", e))
    });

    tokio::runtime::run(work);
}
