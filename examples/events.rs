extern crate async_docker;
extern crate futures;
extern crate http;
extern crate tokio;

use async_docker::{new_docker, DockerApi, EventsOptionsBuilder};
use futures::{future, Future, Stream};

fn main() {
    let work = future::lazy(|| {
        let opts = EventsOptionsBuilder::default().build();
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        docker
            .events(&opts)
            .for_each(|a| Ok(println!("{:?}", a)))
            .map_err(|e| eprintln!("{:?}", e))
    });

    tokio::runtime::run(work);
}
