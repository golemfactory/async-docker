extern crate async_docker;
extern crate futures;
extern crate http;
extern crate tokio;

use async_docker::{new_docker, DockerApi, PullOptions};
use futures::{future, Future, Stream};
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
            .for_each(|a| Ok(println!("{:?}", a)))
            .map_err(|e| eprintln!("{:?}", e))
    });

    tokio::runtime::run(work);
}
