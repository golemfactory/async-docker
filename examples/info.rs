extern crate shiplift;
extern crate http;
extern crate futures;
extern crate tokio;

use shiplift::{DockerApi, new_docker};
use futures::{future, Future};

fn main() {
    let work = future::lazy(||  {
        let docker: Box<DockerApi> = new_docker(None).unwrap();

        docker
            .info()
            .then(|a| Ok(println!("{:?}", a)))
    });

    tokio::runtime::run(work);
}