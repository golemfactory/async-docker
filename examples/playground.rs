extern crate shiplift;
extern crate hyper;
extern crate tokio;
extern crate futures;
extern crate http;

use shiplift::communicate::Container;
use shiplift::UnixDocker;
use shiplift::DockerTrait;
use hyper::rt::Future;
use futures::future;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;
use shiplift::build::LogsOptions;
use futures::Stream;
use tokio::reactor::Reactor;
use tokio::executor::current_thread::CurrentThread;
use futures::executor::Spawn;
use tokio::executor::DefaultExecutor;
use tokio::executor::Executor;


fn main() {
    let uri : http::uri::Uri = "unix://var/run/docker.sock".parse().unwrap();
    println!("{:?}", uri);

    let work = future::lazy(||  {
        let docker = UnixDocker::new(uri).unwrap();

        //let _opts = shiplift::build::EventsOptions::default();
        let options = shiplift::ExecContainerOptions::builder()
            .cmd(vec![
                "bash",
                "-c",
                "echo -n \"echo VAR=$VAR on stdout\"; echo -n \"echo VAR=$VAR on stderr\" >&2",
            ])
            .env(vec!["VAR=value"])
            .attach_stdout(true)
            .attach_stderr(true)
            .build();
        //let mut a = docker.container("bda315b5d0ba").stats();

        let reactor = Reactor::new();

        docker
            .container("c44b472a64bc")
            .exec(&options)
            .for_each(|a| Ok(println!("{:?}", a)))
            .map_err(|e| println!("{:?}", e))
    });

    tokio::runtime::run(work);
}
