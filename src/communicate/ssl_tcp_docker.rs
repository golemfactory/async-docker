#![cfg(feature = "ssl")]

extern crate hyper_openssl;
extern crate openssl;
use self::hyper_openssl::HttpsConnector;
use self::openssl::ssl::SslConnectorBuilder;
use self::openssl::x509::X509_FILETYPE_PEM;
use self::openssl::ssl::SslMethod;

use util::docker::Docker;
use hyper::Uri;
use util::docker::DockerTrait;
use errors::Result;
use Error;
use std::path::Path;
use std::env;
use hyper::Client;
use communicate::DockerTrait;
use communicate::docker::Docker;
use std::sync::Arc;
use hyper::client::HttpConnector;
use hyper::Body;
use transport::interact::Interact;
use communicate::docker::DockerApi;

pub type TcpSSLDocker = Docker<HttpsConnector<HttpConnector>>;

const THREADS: usize = 1;

impl DockerTrait for Docker<HttpsConnector<HttpConnector>> {
    type Connector = HttpsConnector<HttpConnector>;

    fn new(host: Uri) -> Result<Box<DockerApi>> {
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

        let mut http = HttpConnector::new(THREADS);
        http.enforce_http(false);

        let connector = HttpsConnector::<HttpConnector>::with_connector(http, connector)
            .map_err(Error::from)?;

        let client = Client::builder().build(connector);


        let docker = Self::new_inner(Arc::new(Interact::new(client, host)));
        Ok(Box::new(docker))
    }
}

