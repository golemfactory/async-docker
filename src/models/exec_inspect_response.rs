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
pub struct ExecInspectResponse {
    #[serde(rename = "CanRemove")]
    can_remove: Option<bool>,
    #[serde(rename = "DetachKeys")]
    detach_keys: Option<String>,
    #[serde(rename = "ID")]
    ID: Option<String>,
    #[serde(rename = "Running")]
    running: Option<bool>,
    #[serde(rename = "ExitCode")]
    exit_code: Option<i32>,
    #[serde(rename = "ProcessConfig")]
    process_config: Option<::models::ProcessConfig>,
    #[serde(rename = "OpenStdin")]
    open_stdin: Option<bool>,
    #[serde(rename = "OpenStderr")]
    open_stderr: Option<bool>,
    #[serde(rename = "OpenStdout")]
    open_stdout: Option<bool>,
    #[serde(rename = "ContainerID")]
    container_id: Option<String>,
    /// The system process ID for the exec process.
    #[serde(rename = "Pid")]
    pid: Option<i32>,
}

impl ExecInspectResponse {
    pub fn new() -> ExecInspectResponse {
        ExecInspectResponse {
            can_remove: None,
            detach_keys: None,
            ID: None,
            running: None,
            exit_code: None,
            process_config: None,
            open_stdin: None,
            open_stderr: None,
            open_stdout: None,
            container_id: None,
            pid: None,
        }
    }

    pub fn set_can_remove(&mut self, can_remove: bool) {
        self.can_remove = Some(can_remove);
    }

    pub fn with_can_remove(mut self, can_remove: bool) -> ExecInspectResponse {
        self.can_remove = Some(can_remove);
        self
    }

    pub fn can_remove(&self) -> Option<&bool> {
        self.can_remove.as_ref()
    }

    pub fn reset_can_remove(&mut self) {
        self.can_remove = None;
    }

    pub fn set_detach_keys(&mut self, detach_keys: String) {
        self.detach_keys = Some(detach_keys);
    }

    pub fn with_detach_keys(mut self, detach_keys: String) -> ExecInspectResponse {
        self.detach_keys = Some(detach_keys);
        self
    }

    pub fn detach_keys(&self) -> Option<&String> {
        self.detach_keys.as_ref()
    }

    pub fn reset_detach_keys(&mut self) {
        self.detach_keys = None;
    }

    pub fn set_ID(&mut self, ID: String) {
        self.ID = Some(ID);
    }

    pub fn with_ID(mut self, ID: String) -> ExecInspectResponse {
        self.ID = Some(ID);
        self
    }

    pub fn ID(&self) -> Option<&String> {
        self.ID.as_ref()
    }

    pub fn reset_ID(&mut self) {
        self.ID = None;
    }

    pub fn set_running(&mut self, running: bool) {
        self.running = Some(running);
    }

    pub fn with_running(mut self, running: bool) -> ExecInspectResponse {
        self.running = Some(running);
        self
    }

    pub fn running(&self) -> Option<&bool> {
        self.running.as_ref()
    }

    pub fn reset_running(&mut self) {
        self.running = None;
    }

    pub fn set_exit_code(&mut self, exit_code: i32) {
        self.exit_code = Some(exit_code);
    }

    pub fn with_exit_code(mut self, exit_code: i32) -> ExecInspectResponse {
        self.exit_code = Some(exit_code);
        self
    }

    pub fn exit_code(&self) -> Option<&i32> {
        self.exit_code.as_ref()
    }

    pub fn reset_exit_code(&mut self) {
        self.exit_code = None;
    }

    pub fn set_process_config(&mut self, process_config: ::models::ProcessConfig) {
        self.process_config = Some(process_config);
    }

    pub fn with_process_config(
        mut self,
        process_config: ::models::ProcessConfig,
    ) -> ExecInspectResponse {
        self.process_config = Some(process_config);
        self
    }

    pub fn process_config(&self) -> Option<&::models::ProcessConfig> {
        self.process_config.as_ref()
    }

    pub fn reset_process_config(&mut self) {
        self.process_config = None;
    }

    pub fn set_open_stdin(&mut self, open_stdin: bool) {
        self.open_stdin = Some(open_stdin);
    }

    pub fn with_open_stdin(mut self, open_stdin: bool) -> ExecInspectResponse {
        self.open_stdin = Some(open_stdin);
        self
    }

    pub fn open_stdin(&self) -> Option<&bool> {
        self.open_stdin.as_ref()
    }

    pub fn reset_open_stdin(&mut self) {
        self.open_stdin = None;
    }

    pub fn set_open_stderr(&mut self, open_stderr: bool) {
        self.open_stderr = Some(open_stderr);
    }

    pub fn with_open_stderr(mut self, open_stderr: bool) -> ExecInspectResponse {
        self.open_stderr = Some(open_stderr);
        self
    }

    pub fn open_stderr(&self) -> Option<&bool> {
        self.open_stderr.as_ref()
    }

    pub fn reset_open_stderr(&mut self) {
        self.open_stderr = None;
    }

    pub fn set_open_stdout(&mut self, open_stdout: bool) {
        self.open_stdout = Some(open_stdout);
    }

    pub fn with_open_stdout(mut self, open_stdout: bool) -> ExecInspectResponse {
        self.open_stdout = Some(open_stdout);
        self
    }

    pub fn open_stdout(&self) -> Option<&bool> {
        self.open_stdout.as_ref()
    }

    pub fn reset_open_stdout(&mut self) {
        self.open_stdout = None;
    }

    pub fn set_container_id(&mut self, container_id: String) {
        self.container_id = Some(container_id);
    }

    pub fn with_container_id(mut self, container_id: String) -> ExecInspectResponse {
        self.container_id = Some(container_id);
        self
    }

    pub fn container_id(&self) -> Option<&String> {
        self.container_id.as_ref()
    }

    pub fn reset_container_id(&mut self) {
        self.container_id = None;
    }

    pub fn set_pid(&mut self, pid: i32) {
        self.pid = Some(pid);
    }

    pub fn with_pid(mut self, pid: i32) -> ExecInspectResponse {
        self.pid = Some(pid);
        self
    }

    pub fn pid(&self) -> Option<&i32> {
        self.pid.as_ref()
    }

    pub fn reset_pid(&mut self) {
        self.pid = None;
    }
}
