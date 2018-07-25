extern crate shiplift;
extern crate hyper;
extern crate tokio;
extern crate futures;
extern crate http;

use std::str::FromStr;

use shiplift::Docker;
use shiplift::UnixDocker;
use shiplift::DockerTrait;
use shiplift::rep::Info;
use hyper::rt::Future;
use futures::future;
use std::io;
use std::io::Write;
use tokio::timer::Deadline;
use tokio::timer::Delay;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::executor::Executor;

fn main() {
    let uri : http::uri::Uri = "unix://var/run/docker.sock".parse().unwrap();

    println!("{:?}", uri);

    let docker = UnixDocker::new(uri);


    let connection = docker.unwrap().version()
        .then(|res| {
            println!("wrote message; success={:?}", res);
            Ok(())
        });

    let mut rt = Runtime::new().unwrap();
    let executor = rt.executor();

    // Spawn a new task that processes the socket:
    let a = executor.spawn(connection);


    thread::sleep(Duration::from_millis(200));
    //tokio::run(c);
    //println!("info {:?}", docker.info().unwrap());
}
