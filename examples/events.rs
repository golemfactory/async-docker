extern crate shiplift;
extern crate http;
extern crate futures;
extern crate tokio;

use shiplift::{DockerApi, new_docker, EventsOptionsBuilder};
use futures::{future, Stream, Future};

fn main() {
    let work = future::lazy(||  {
        let opts = EventsOptionsBuilder::default().build();
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        docker
            .events(&opts)
            .for_each(|a| Ok(println!("{:?}", a)))
            .map_err(|e| eprintln!("{:?}", e))
    });

    tokio::runtime::run(work);
}