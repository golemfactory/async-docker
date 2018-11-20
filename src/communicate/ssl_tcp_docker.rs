#![cfg(feature = "ssl")]

extern crate hyper_openssl;
extern crate openssl;
use self::{
    hyper_openssl::HttpsConnector,
    openssl::{
        ssl::{SslConnectorBuilder, SslMethod},
        x509::X509Builder,
    },
};

use self::openssl::ssl::SslContextBuilder;
use communicate::docker::{Docker, DockerApi};
use errors::Result;
use hyper::{client::HttpConnector, Body, Client, Uri};
use std::{env, path::Path, sync::Arc};
use transport::interact::Interact;
use Error;

pub type TcpSSLDocker = Docker<HttpsConnector<HttpConnector>>;

const THREADS: usize = 1;

impl Docker<HttpsConnector<HttpConnector>> {
    pub(crate) fn new(host: Uri) -> Result<Box<DockerApi>> {
        let certs = env::var("DOCKER_CERT_PATH").ok().expect("No SSL cert");

        let cert = &format!("{}/cert.pem", certs);
        let key = &format!("{}/key.pem", certs);

        // https://github.com/hyperium/hyper/blob/master/src/net.rs#L427-L428
        let context = SslContextBuilder::new(SslMethod::tls())
            .expect("Error during ssl connection preparation");
        let mut connector = SslConnectorBuilder { context }.build();

        connector.set_cipher_list("DEFAULT")?;
        connector.set_certificate_file(&Path::new(cert), X509Builder::new())?;
        connector.set_private_key_file(&Path::new(key), X509Builder::new())?;

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
