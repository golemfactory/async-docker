extern crate async_docker;
extern crate futures;
extern crate http;
extern crate serde_json;
extern crate tokio;

use async_docker::{
    communicate::{new_docker, DockerApi},
    models::{ContainerConfig, HostConfig},
    PullOptions,
};
use futures::{future, Future, Stream};
use std::{borrow::Cow, env};

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
            .with_image(image.clone())
            .with_env(vec!["BLENDER_DEVICE_TYPE=cpu".to_string()])
            .with_host_config(HostConfig::new().with_binds(vec![
                pwd.clone() + "/resources:/golem/resources",
                pwd.clone() + "/output:/golem/output",
                pwd + "/work:/golem/work",
            ]))
            .with_cmd(vec!["/golem/work/job.py".into()]);

        let pull_opts = PullOptions::builder()
            .image("golemfactory/blender")
            .tag("1.5")
            .build();

        docker
            .images()
            .pull(&pull_opts)
            .for_each(|a| Ok(println!("{:?}", a)))
            .and_then(move |_| {
                docker
                    .containers()
                    .create(&opts)
                    .and_then(move |a| {
                        let id = a.id().to_owned();
                        Ok(docker.container(Cow::<'static>::from(id)))
                    })
                    .and_then(|container| {
                        let container2 = container.clone();
                        container
                            .start()
                            .and_then(move |_| container.wait())
                            .then(move |x| container2.delete().then(|_| x))
                    })
            })
            .map(|x| println!("Status code of the job: {}", x))
            .map_err(|e| println!("Err: {}", e))
            .then(|_| Ok(()))
    });

    tokio::runtime::run(work);
}
