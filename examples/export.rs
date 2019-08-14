extern crate async_docker;
extern crate futures;
extern crate http;
extern crate tokio;

use async_docker::{new_docker, DockerApi};
use futures::Stream;
use futures::{future, Future};
use std::env;
use std::fs::OpenOptions;
use std::io::copy;

fn main() {
    if env::args().count() < 2 {
        println!("Too few arguments (<1).");
        return;
    }

    let image_id = env::args().nth(1).unwrap();

    let work = future::lazy(|| {
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        let mut export = OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{}.tgz", &image_id))
            .unwrap();

        docker
            .image(image_id.into())
            .export()
            .concat2()
            .and_then(move |chunk| {
                let strin = chunk.into_bytes();
                copy(&mut strin.as_ref(), &mut export).unwrap();
                Ok(println!("Success"))
            })
            .map_err(|a| eprintln!("{:?}", a))
    });

    tokio::runtime::run(work);
}
