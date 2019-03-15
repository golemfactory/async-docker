#![cfg(feature = "ssl")]

extern crate hyper_openssl;
extern crate openssl;
use self::hyper_openssl::HttpsConnector;
use self::openssl::ssl::SslMethod;

use self::openssl::ssl::SslContextBuilder;
use communicate::docker::Docker;
use communicate::docker::DockerApi;
use errors::Result;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper::Uri;
use std::env;
use std::path::Path;
use std::sync::Arc;
use transport::interact::Interact;
use Error;
use self::openssl::ssl::SslConnector;
use self::openssl::ssl::SslFiletype;

pub(crate) type TcpSSLDocker = Docker<HttpsConnector<HttpConnector>>;

const THREADS: usize = 1;

impl Docker<HttpsConnector<HttpConnector>> {
    pub(crate) fn new(host: Uri) -> Result<Box<DockerApi>> {
        let certs = env::var("DOCKER_CERT_PATH").ok().expect("No SSL cert");

        let cert = &format!("{}/cert.pem", certs);
        let key = &format!("{}/key.pem", certs);

        // https://github.com/hyperium/hyper/blob/master/src/net.rs#L427-L428
        let mut connector = SslConnector::builder(SslMethod::tls())
            .expect("Error during ssl connection preparation");

        connector.set_cipher_list("DEFAULT")?;
        connector.set_certificate_file(&Path::new(cert), SslFiletype::PEM)?;
        connector.set_private_key_file(&Path::new(key), SslFiletype::PEM)?;

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
