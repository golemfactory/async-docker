extern crate async_docker;
extern crate futures;
extern crate http;
extern crate tokio;

use async_docker::communicate::new_docker;
use async_docker::communicate::DockerApi;
use futures::future;
use futures::Future;
use futures::Stream;
use std::env;

fn main() {
    if env::args().count() < 2 {
        println!("Too few arguments (<2).");
        return;
    }

    let container = env::args().nth(1).unwrap();
    let remote_path = env::args().nth(2).unwrap();

    let work = future::lazy(move || {
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        docker
            .container(container.into())
            .archive_get(remote_path.as_str())
            .for_each(|a| Ok(println!("{:?}", a)))
            .map_err(|a| eprintln!("{:?}", a))
    });

    tokio::runtime::run(work);
}
