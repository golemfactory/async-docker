#![cfg(feature = "ssl")]

extern crate hyper_openssl;
extern crate openssl;
use self::hyper_openssl::HttpsConnector;
use self::openssl::ssl::SslConnectorBuilder;
use self::openssl::x509::X509_FILETYPE_PEM;
use self::openssl::ssl::SslMethod;

use structs::docker::Docker;
use hyper::Uri;
use structs::docker::DockerTrait;
use errors::Result;
use std::path::Path;
use std::env;
use hyper::Client;

pub type TcpSSLDocker = Docker<HttpsConnector<OpensslClient>>;

impl DockerTrait for Docker<HttpsConnector<OpensslClient>> {
    type Connector = HttpsConnector<OpensslClient>;

    fn new(host: Uri) -> Result<Self> {
        let Some(certs) = env::var("DOCKER_CERT_PATH").ok()?;

        let cert = &format!("{}/cert.pem", certs);
        let key = &format!("{}/key.pem", certs);

        // https://github.com/hyperium/hyper/blob/master/src/net.rs#L427-L428
        let mut connector = SslConnectorBuilder::new(SslMethod::tls())?;

        connector.set_cipher_list("DEFAULT")?;
        connector.set_certificate_file(&Path::new(cert), X509_FILETYPE_PEM)?;
        connector.set_private_key_file(&Path::new(key), X509_FILETYPE_PEM)?;

        if let Some(_) = env::var("DOCKER_TLS_VERIFY").ok() {
            let ca = &format!("{}/ca.pem", certs);
            connector.set_ca_file(&Path::new(ca))?;
        }

        let ssl = OpensslClient::from(connector.build());
        Client::with_connector(HttpsConnector::new(ssl))
    }

    fn host(&self) -> &Uri {
        &self.host
    }

    fn client(&self) -> &Client<Self::Connector> {
        &self.client
    }
}