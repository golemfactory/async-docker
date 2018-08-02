use hyper::client::HttpConnector;
use hyper::Uri;
use hyper::Client;

use errors::Result;
use communicate::DockerTrait;
use communicate::docker::Docker;
use transport::interact::Interact;
use std::sync::Arc;

pub type TcpDocker = Docker<HttpConnector>;

impl DockerTrait for Docker<HttpConnector> {
    type Connector = HttpConnector;

    fn new(host: Uri) -> Result<Self> {
        Ok(TcpDocker { interact: Arc::new(
            Interact::new(Client::new(), host)
        )})
    }

    fn interact(&self) -> Arc<Interact<Self::Connector>> {
        self.interact.clone()
    }
}