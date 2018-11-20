use hyper::{client::HttpConnector, Client, Uri};

use communicate::docker::{Docker, DockerApi};
use errors::Result;
use std::sync::Arc;
use transport::interact::Interact;

pub(super) type TcpDocker = Docker<HttpConnector>;

impl Docker<HttpConnector> {
    pub(crate) fn new(host: Uri) -> Result<Box<DockerApi>> {
        let interact = Interact::new(Client::new(), host);
        let docker = Self::new_inner(Arc::new(interact));

        Ok(Box::new(docker))
    }
}
