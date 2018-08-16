extern crate async_docker;
extern crate http;
extern crate futures;
extern crate tokio;

use async_docker::{DockerApi, new_docker, ExecContainerOptions};
use futures::{future, Future, Stream};
use std::env;

fn get_opts() -> ExecContainerOptions {
    ExecContainerOptions::builder()
        .cmd(vec![
            "bash",
            "-c",
            "echo -n \"echo VAR=$VAR on stdout\"; echo -n \"echo VAR=$VAR on stderr\" >&2",
        ])
        .env(vec!["VAR=value"])
        .attach_stdout(true)
        .attach_stderr(true)
        .build()
}

fn main() {
    let id = match env::args().nth(1) {
        Some(val) => val,
        None => {
            println!("Not enough arguments");
            return;
        }
    };

    let work = future::lazy(move||  {
        let docker: Box<DockerApi> = new_docker(None).unwrap();
        docker
            .container(id.into())
            .exec(&get_opts())
            .for_each(|a| Ok(println!("{:?}", a)))
            .map_err(|e| println!("{:#?}", e))
    });

    tokio::runtime::run(work);
}