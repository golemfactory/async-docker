extern crate shiplift;
extern crate http;
extern crate futures;
extern crate tokio;


use std::env;
use shiplift::communicate::DockerApi;
use shiplift::communicate::new_docker;
use futures::future;
use futures::Future;
use shiplift::ContainerOptions;

fn main() {
    if env::args().count() < 2 {
        println!("Too few arguments (<1).");
        return;
    }

    let image = env::args().nth(1).unwrap();

    let work = future::lazy(move || {
        let docker: Box<DockerApi> = new_docker(None).unwrap();
        let opts = ContainerOptions::builder(image.as_ref()).build();

        docker
            .containers()
            .create(&opts)
            .and_then(|a| Ok(println!("{:?}", a)))
            .map_err(|a| eprintln!("{:?}", a))
    });

    tokio::runtime::run(work);
}
