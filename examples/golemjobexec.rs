extern crate async_docker;
extern crate futures;
extern crate http;
extern crate serde_json;
extern crate tokio;

use async_docker::models::HostConfig;
use async_docker::{
    communicate::{new_docker, DockerApi},
    models::ContainerConfig,
};
use futures::{future, Future};
use std::borrow::Cow;
use std::env;

fn main() {
    if env::args().count() < 2 {
        println!("Too few arguments (<1).");
        return;
    }

    match env::var("PWD") {
        Err(_) => println!("env variable PWD not set"),
        _ => (),
    }

    let pwd = env::var("PWD").unwrap();
    let image = env::args().nth(1).unwrap();

    let work = future::lazy(move || {
        let docker: Box<DockerApi> = new_docker(None).unwrap();
        let opts = ContainerConfig::new()
            .with_image(image)
            .with_env(vec!["BLENDER_DEVICE_TYPE=cpu".to_string()])
            .with_host_config(HostConfig::new().with_binds(vec![
                pwd.clone() + "/resources:/golem/resources",
                pwd.clone() + "/output:/golem/output",
                pwd + "/work:/golem/work",
            ]))
            .with_cmd(vec!["/golem/work/job.py".into()]);
        println!("{:?}", serde_json::to_string(&opts));

        docker
            .containers()
            .create(&opts)
            .and_then(move |a| {
                let id = a.id().to_owned();
                Ok(docker.container(Cow::<'static>::from(id)))
            })
            .and_then(|container| {
                container
                    .start()
                    .and_then(move |_| container.wait().and_then(move |_| container.delete()))
            })
            .then(|_| Ok(()))
    });

    tokio::runtime::run(work);
}
