extern crate shiplift;
extern crate hyper;
extern crate tokio;
extern crate futures;
extern crate http;

use shiplift::communicate::Container;
use hyper::rt::Future;
use futures::future;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;
use shiplift::build::LogsOptions;
use shiplift::build::ContainerArchiveOptions;
use futures::Stream;
use tokio::reactor::Reactor;
use tokio::executor::current_thread::CurrentThread;
use futures::executor::Spawn;
use tokio::executor::DefaultExecutor;
use tokio::executor::Executor;
use shiplift::communicate::docker::new_docker;
use shiplift::communicate::docker::DockerApi;


fn main() {
    let uri : http::uri::Uri = "unix://var/run/docker.sock".parse().unwrap();
    println!("{:?}", uri);

    let work = future::lazy(||  {
        let docker: Box<DockerApi> = new_docker(uri).unwrap();

        docker
            .version()
            .then(|a| Ok(println!("{:?}", a)))
    });

    tokio::runtime::run(work);
}
