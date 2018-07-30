use ::docker::Docker;
use ::docker::DockerTrait;

use hyper::client::HttpConnector;
use hyper::Uri;
use hyper::Client;

use errors::Result;

pub type TcpDocker = Docker<HttpConnector>;

impl DockerTrait for Docker<HttpConnector> {
    type Connector = HttpConnector;

    fn new(host: Uri) -> Result<Self> {
        Ok(TcpDocker {
            client: Client::new(),
            host
        })
    }

    fn host(&self) -> &Uri {
        &self.host
    }

    fn client(&self) -> &Client<Self::Connector> {
        &self.client
    }
}