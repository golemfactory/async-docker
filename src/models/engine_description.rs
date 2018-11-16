/*
 * Docker Engine API
 *
 * The Engine API is an HTTP API served by Docker Engine. It is the API the Docker client uses to communicate with the Engine, so everything the Docker client can do can be done with the API.  Most of the client's commands map directly to API endpoints (e.g. `docker ps` is `GET /containers/json`). The notable exception is running containers, which consists of several API calls.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call. The body of the response will be JSON in the following format:  ``` {   \"message\": \"page not found\" } ```  # Versioning  The API is usually changed in each release, so API calls are versioned to ensure that clients don't break. To lock to a specific version of the API, you prefix the URL with its version, for example, call `/v1.30/info` to use the v1.30 version of the `/info` endpoint. If the API version specified in the URL is not supported by the daemon, a HTTP `400 Bad Request` error message is returned.  If you omit the version-prefix, the current version of the API (v1.40) is used. For example, calling `/info` is the same as calling `/v1.40/info`. Using the API without a version-prefix is deprecated and will be removed in a future release.  Engine releases in the near future should support this version of the API, so your client will continue to work even if it is talking to a newer Engine.  The API uses an open schema model, which means server may add extra properties to responses. Likewise, the server will ignore any extra query parameters and request body properties. When you write clients, you need to ignore additional properties in responses to ensure they do not break when talking to newer daemons.   # Authentication  Authentication for registries is handled client side. The client has to send authentication details to various endpoints that need to communicate with registries, such as `POST /images/(name)/push`. These are sent as `X-Registry-Auth` header as a Base64 encoded (JSON) string with the following structure:  ``` {   \"username\": \"string\",   \"password\": \"string\",   \"email\": \"string\",   \"serveraddress\": \"string\" } ```  The `serveraddress` is a domain/IP without a protocol. Throughout this structure, double quotes are required.  If you have already got an identity token from the [`/auth` endpoint](#operation/SystemAuth), you can just pass this instead of credentials:  ``` {   \"identitytoken\": \"9cbaf023786cd7...\" } ```
 *
 * OpenAPI spec version: 1.40
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// EngineDescription : EngineDescription provides information about an engine.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineDescription {
    #[serde(rename = "EngineVersion")]
    engine_version: Option<String>,
    #[serde(rename = "Labels")]
    labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Plugins")]
    plugins: Option<Vec<::models::EngineDescriptionPlugins>>,
}

impl EngineDescription {
    /// EngineDescription provides information about an engine.
    pub fn new() -> EngineDescription {
        EngineDescription {
            engine_version: None,
            labels: None,
            plugins: None,
        }
    }

    pub fn set_engine_version(&mut self, engine_version: String) {
        self.engine_version = Some(engine_version);
    }

    pub fn with_engine_version(mut self, engine_version: String) -> EngineDescription {
        self.engine_version = Some(engine_version);
        self
    }

    pub fn engine_version(&self) -> Option<&String> {
        self.engine_version.as_ref()
    }

    pub fn reset_engine_version(&mut self) {
        self.engine_version = None;
    }

    pub fn set_labels(&mut self, labels: ::std::collections::HashMap<String, String>) {
        self.labels = Some(labels);
    }

    pub fn with_labels(
        mut self,
        labels: ::std::collections::HashMap<String, String>,
    ) -> EngineDescription {
        self.labels = Some(labels);
        self
    }

    pub fn labels(&self) -> Option<&::std::collections::HashMap<String, String>> {
        self.labels.as_ref()
    }

    pub fn reset_labels(&mut self) {
        self.labels = None;
    }

    pub fn set_plugins(&mut self, plugins: Vec<::models::EngineDescriptionPlugins>) {
        self.plugins = Some(plugins);
    }

    pub fn with_plugins(
        mut self,
        plugins: Vec<::models::EngineDescriptionPlugins>,
    ) -> EngineDescription {
        self.plugins = Some(plugins);
        self
    }

    pub fn plugins(&self) -> Option<&Vec<::models::EngineDescriptionPlugins>> {
        self.plugins.as_ref()
    }

    pub fn reset_plugins(&mut self) {
        self.plugins = None;
    }
}
