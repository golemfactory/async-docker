/*
 * Docker Engine API
 *
 * The Engine API is an HTTP API served by Docker Engine. It is the API the Docker client uses to communicate with the Engine, so everything the Docker client can do can be done with the API.  Most of the client's commands map directly to API endpoints (e.g. `docker ps` is `GET /containers/json`). The notable exception is running containers, which consists of several API calls.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call. The body of the response will be JSON in the following format:  ``` {   \"message\": \"page not found\" } ```  # Versioning  The API is usually changed in each release, so API calls are versioned to ensure that clients don't break. To lock to a specific version of the API, you prefix the URL with its version, for example, call `/v1.30/info` to use the v1.30 version of the `/info` endpoint. If the API version specified in the URL is not supported by the daemon, a HTTP `400 Bad Request` error message is returned.  If you omit the version-prefix, the current version of the API (v1.40) is used. For example, calling `/info` is the same as calling `/v1.40/info`. Using the API without a version-prefix is deprecated and will be removed in a future release.  Engine releases in the near future should support this version of the API, so your client will continue to work even if it is talking to a newer Engine.  The API uses an open schema model, which means server may add extra properties to responses. Likewise, the server will ignore any extra query parameters and request body properties. When you write clients, you need to ignore additional properties in responses to ensure they do not break when talking to newer daemons.   # Authentication  Authentication for registries is handled client side. The client has to send authentication details to various endpoints that need to communicate with registries, such as `POST /images/(name)/push`. These are sent as `X-Registry-Auth` header as a Base64 encoded (JSON) string with the following structure:  ``` {   \"username\": \"string\",   \"password\": \"string\",   \"email\": \"string\",   \"serveraddress\": \"string\" } ```  The `serveraddress` is a domain/IP without a protocol. Throughout this structure, double quotes are required.  If you have already got an identity token from the [`/auth` endpoint](#operation/SystemAuth), you can just pass this instead of credentials:  ``` {   \"identitytoken\": \"9cbaf023786cd7...\" } ```
 *
 * OpenAPI spec version: 1.40
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretSpec {
    /// User-defined name of the secret.
    #[serde(rename = "Name")]
    name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    labels: Option<::std::collections::HashMap<String, String>>,
    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-3.2)) data to store as secret.  This field is only used to _create_ a secret, and is not returned by other endpoints.
    #[serde(rename = "Data")]
    data: Option<String>,
    /// Name of the secrets driver used to fetch the secret's value from an external secret store
    #[serde(rename = "Driver")]
    driver: Option<::models::Driver>,
    /// Templating driver, if applicable  Templating controls whether and how to evaluate the config payload as a template. If no driver is set, no templating is used.
    #[serde(rename = "Templating")]
    templating: Option<::models::Driver>,
}

impl SecretSpec {
    pub fn new() -> SecretSpec {
        SecretSpec {
            name: None,
            labels: None,
            data: None,
            driver: None,
            templating: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: String) -> SecretSpec {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_labels(&mut self, labels: ::std::collections::HashMap<String, String>) {
        self.labels = Some(labels);
    }

    pub fn with_labels(
        mut self,
        labels: ::std::collections::HashMap<String, String>,
    ) -> SecretSpec {
        self.labels = Some(labels);
        self
    }

    pub fn labels(&self) -> Option<&::std::collections::HashMap<String, String>> {
        self.labels.as_ref()
    }

    pub fn reset_labels(&mut self) {
        self.labels = None;
    }

    pub fn set_data(&mut self, data: String) {
        self.data = Some(data);
    }

    pub fn with_data(mut self, data: String) -> SecretSpec {
        self.data = Some(data);
        self
    }

    pub fn data(&self) -> Option<&String> {
        self.data.as_ref()
    }

    pub fn reset_data(&mut self) {
        self.data = None;
    }

    pub fn set_driver(&mut self, driver: ::models::Driver) {
        self.driver = Some(driver);
    }

    pub fn with_driver(mut self, driver: ::models::Driver) -> SecretSpec {
        self.driver = Some(driver);
        self
    }

    pub fn driver(&self) -> Option<&::models::Driver> {
        self.driver.as_ref()
    }

    pub fn reset_driver(&mut self) {
        self.driver = None;
    }

    pub fn set_templating(&mut self, templating: ::models::Driver) {
        self.templating = Some(templating);
    }

    pub fn with_templating(mut self, templating: ::models::Driver) -> SecretSpec {
        self.templating = Some(templating);
        self
    }

    pub fn templating(&self) -> Option<&::models::Driver> {
        self.templating.as_ref()
    }

    pub fn reset_templating(&mut self) {
        self.templating = None;
    }
}