use hyper::client::HttpConnector;
use hyper::Uri;
use hyper::Client;

use errors::Result;
use communicate::docker::Docker;
use communicate::docker::DockerApi;
use transport::interact::Interact;
use std::sync::Arc;
use std::marker::PhantomData;

pub(super) type TcpDocker = Docker<HttpConnector>;

impl Docker<HttpConnector> {
    pub(crate) fn new(host: Uri) -> Result<Box<DockerApi>> {
        let interact = Interact::new(Client::new(), host);
        let docker = Self::new_inner(Arc::new(interact));

        Ok(Box::new(docker))
    }
}