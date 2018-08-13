extern crate shiplift;
extern crate http;
extern crate futures;
extern crate tokio;


use std::env;
use shiplift::communicate::DockerApi;
use shiplift::communicate::new_docker;
use futures::future;
use futures::Future;
use shiplift::ContainerConnectionOptions;

fn main() {
    if env::args().count() < 3 {
        println!("Too few arguments (<2).");
        return;
    }

    let container_id = env::args().nth(1).unwrap();
    let network_id = env::args().nth(2).unwrap();

    let work = future::lazy(move || {
        let docker: Box<DockerApi> = new_docker(None).unwrap();
        let opts = ContainerConnectionOptions::new(&container_id);

        docker
            .network(network_id.into())
            .connect(&opts)
            .and_then(|a| Ok(println!("{:?}", a)))
            .map_err(|a| eprintln!("{:?}", a))
    });

    tokio::runtime::run(work);
}
