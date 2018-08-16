extern crate async_docker;
extern crate http;
extern crate futures;
extern crate tokio;


use std::env;
use async_docker::communicate::DockerApi;
use async_docker::communicate::new_docker;
use futures::future;
use futures::Future;
fn main() {
    if env::args().count() < 2 {
        println!("Too few arguments (<1).");
        return;
    }

    let container_id  = env::args().nth(1).unwrap();

    let work = future::lazy(move || {
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        docker
            .container(container_id.into())
            .top(Default::default())
            .and_then(|a| Ok(println!("{:?}", a)))
            .map_err(|a| eprintln!("{:?}", a))
    });

    tokio::runtime::run(work);
}
