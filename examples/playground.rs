extern crate shiplift;
extern crate hyper;
extern crate tokio;
extern crate futures;
extern crate http;

use std::str::FromStr;

use shiplift::Container;
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
use shiplift::structs::UnixConnector;
use futures::Stream;
use futures::Async;

fn main() {
    let uri : http::uri::Uri = "unix://var/run/docker.sock".parse().unwrap();

    println!("{:?}", uri);

    let docker = UnixDocker::new(uri).unwrap();


    let opts = shiplift::builder::EventsOptions::default();
    //let mut a = docker.events(&opts);
/*
    let a = future.and_then(|stream| {
        stream.
            for_each(|b| {
                println!("wrote message; success={:?}", b);
            });
        Ok(())
    });
*/
    let container = Container::new(docker, "b89d18ebad39");
    let mut a = container.stats();
    let do_it = futures::lazy(|| {
        // existing loop goes here
        println!("start");
        a
            .for_each(|b| {
                println!("wrote message; success={:?}", b);
                Ok(())
            }).and_then(|_| Ok(println!("done")))
            .map_err(|_| ())

    });

    let rt = Runtime::new().unwrap();
    let executor = rt.executor();

    // Spawn a new task that processes the socket:
    let _ = executor.spawn(do_it);


    thread::sleep(Duration::from_millis(20000000));
    //tokio::run(c);
    //println!("info {:?}", docker.info().unwrap());

}
