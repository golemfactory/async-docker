extern crate async_docker;
extern crate futures;
extern crate http;
extern crate tokio;

use async_docker::models::ExecConfig;
use async_docker::{new_docker, DockerApi, ExecContainerOptions};
use futures::{future, Future, Stream};
use std::env;

fn get_opts() -> ExecConfig {
    ExecConfig::new()
        .with_cmd(vec![
            "bash".to_string(),
            "-c".to_string(),
            "echo -n \"echo VAR=$VAR on stdout\"; echo -n \"echo VAR=$VAR on stderr\" >&2"
                .to_string(),
        ]).with_env(vec!["VAR=value".to_string()])
        .with_attach_stderr(true)
        .with_attach_stdout(true)
}

fn main() {
    let id = match env::args().nth(1) {
        Some(val) => val,
        None => {
            println!("Not enough arguments");
            return;
        }
    };

    let work = future::lazy(move || {
        let docker: Box<DockerApi> = new_docker(None).unwrap();
        docker
            .container(id.into())
            .exec(&get_opts())
            .for_each(|a| Ok(println!("{:?}", a)))
            .map_err(|e| println!("{:#?}", e))
    });

    tokio::runtime::run(work);
}
