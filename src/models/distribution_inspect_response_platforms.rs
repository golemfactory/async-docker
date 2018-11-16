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
pub struct DistributionInspectResponsePlatforms {
    #[serde(rename = "Architecture")]
    architecture: Option<String>,
    #[serde(rename = "OS")]
    OS: Option<String>,
    #[serde(rename = "OSVersion")]
    os_version: Option<String>,
    #[serde(rename = "OSFeatures")]
    os_features: Option<Vec<String>>,
    #[serde(rename = "Variant")]
    variant: Option<String>,
    #[serde(rename = "Features")]
    features: Option<Vec<String>>,
}

impl DistributionInspectResponsePlatforms {
    pub fn new() -> DistributionInspectResponsePlatforms {
        DistributionInspectResponsePlatforms {
            architecture: None,
            OS: None,
            os_version: None,
            os_features: None,
            variant: None,
            features: None,
        }
    }

    pub fn set_architecture(&mut self, architecture: String) {
        self.architecture = Some(architecture);
    }

    pub fn with_architecture(
        mut self,
        architecture: String,
    ) -> DistributionInspectResponsePlatforms {
        self.architecture = Some(architecture);
        self
    }

    pub fn architecture(&self) -> Option<&String> {
        self.architecture.as_ref()
    }

    pub fn reset_architecture(&mut self) {
        self.architecture = None;
    }

    pub fn set_OS(&mut self, OS: String) {
        self.OS = Some(OS);
    }

    pub fn with_OS(mut self, OS: String) -> DistributionInspectResponsePlatforms {
        self.OS = Some(OS);
        self
    }

    pub fn OS(&self) -> Option<&String> {
        self.OS.as_ref()
    }

    pub fn reset_OS(&mut self) {
        self.OS = None;
    }

    pub fn set_os_version(&mut self, os_version: String) {
        self.os_version = Some(os_version);
    }

    pub fn with_os_version(mut self, os_version: String) -> DistributionInspectResponsePlatforms {
        self.os_version = Some(os_version);
        self
    }

    pub fn os_version(&self) -> Option<&String> {
        self.os_version.as_ref()
    }

    pub fn reset_os_version(&mut self) {
        self.os_version = None;
    }

    pub fn set_os_features(&mut self, os_features: Vec<String>) {
        self.os_features = Some(os_features);
    }

    pub fn with_os_features(
        mut self,
        os_features: Vec<String>,
    ) -> DistributionInspectResponsePlatforms {
        self.os_features = Some(os_features);
        self
    }

    pub fn os_features(&self) -> Option<&Vec<String>> {
        self.os_features.as_ref()
    }

    pub fn reset_os_features(&mut self) {
        self.os_features = None;
    }

    pub fn set_variant(&mut self, variant: String) {
        self.variant = Some(variant);
    }

    pub fn with_variant(mut self, variant: String) -> DistributionInspectResponsePlatforms {
        self.variant = Some(variant);
        self
    }

    pub fn variant(&self) -> Option<&String> {
        self.variant.as_ref()
    }

    pub fn reset_variant(&mut self) {
        self.variant = None;
    }

    pub fn set_features(&mut self, features: Vec<String>) {
        self.features = Some(features);
    }

    pub fn with_features(mut self, features: Vec<String>) -> DistributionInspectResponsePlatforms {
        self.features = Some(features);
        self
    }

    pub fn features(&self) -> Option<&Vec<String>> {
        self.features.as_ref()
    }

    pub fn reset_features(&mut self) {
        self.features = None;
    }
}
