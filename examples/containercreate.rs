extern crate async_docker;
extern crate futures;
extern crate http;
extern crate serde_json;
extern crate tokio;

use async_docker::communicate::new_docker;
use async_docker::communicate::DockerApi;
use async_docker::models::ContainerConfig;
use async_docker::ContainerOptions;
use futures::future;
use futures::Future;
use std::env;

fn main() {
    if env::args().count() < 2 {
        println!("Too few arguments (<1).");
        return;
    }

    let image = env::args().nth(1).unwrap();

    let work = future::lazy(move || {
        let docker: Box<DockerApi> = new_docker(None).unwrap();
        //let opts = ContainerOptions::builder(image.as_ref()).build();
        let opts = ContainerConfig::new().with_image(image);
        println!("{:?}", serde_json::to_string(&opts));

        docker
            .containers()
            .create(&opts)
            .and_then(|a| Ok(println!("{:?}", a)))
            .map_err(|a| eprintln!("{:?}", a))
    });

    tokio::runtime::run(work);
}
