/*
 * Docker Engine API
 *
 * The Engine API is an HTTP API served by Docker Engine. It is the API the Docker client uses to communicate with the Engine, so everything the Docker client can do can be done with the API.  Most of the client's commands map directly to API endpoints (e.g. `docker ps` is `GET /containers/json`). The notable exception is running containers, which consists of several API calls.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call. The body of the response will be JSON in the following format:  ``` {   \"message\": \"page not found\" } ```  # Versioning  The API is usually changed in each release, so API calls are versioned to ensure that clients don't break. To lock to a specific version of the API, you prefix the URL with its version, for example, call `/v1.30/info` to use the v1.30 version of the `/info` endpoint. If the API version specified in the URL is not supported by the daemon, a HTTP `400 Bad Request` error message is returned.  If you omit the version-prefix, the current version of the API (v1.40) is used. For example, calling `/info` is the same as calling `/v1.40/info`. Using the API without a version-prefix is deprecated and will be removed in a future release.  Engine releases in the near future should support this version of the API, so your client will continue to work even if it is talking to a newer Engine.  The API uses an open schema model, which means server may add extra properties to responses. Likewise, the server will ignore any extra query parameters and request body properties. When you write clients, you need to ignore additional properties in responses to ensure they do not break when talking to newer daemons.   # Authentication  Authentication for registries is handled client side. The client has to send authentication details to various endpoints that need to communicate with registries, such as `POST /images/(name)/push`. These are sent as `X-Registry-Auth` header as a Base64 encoded (JSON) string with the following structure:  ``` {   \"username\": \"string\",   \"password\": \"string\",   \"email\": \"string\",   \"serveraddress\": \"string\" } ```  The `serveraddress` is a domain/IP without a protocol. Throughout this structure, double quotes are required.  If you have already got an identity token from the [`/auth` endpoint](#operation/SystemAuth), you can just pass this instead of credentials:  ``` {   \"identitytoken\": \"9cbaf023786cd7...\" } ```
 *
 * OpenAPI spec version: 1.40
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// SwarmSpecRaft : Raft configuration.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SwarmSpecRaft {
    /// The number of log entries between snapshots.
    #[serde(rename = "SnapshotInterval")]
    snapshot_interval: Option<i32>,
    /// The number of snapshots to keep beyond the current snapshot.
    #[serde(rename = "KeepOldSnapshots")]
    keep_old_snapshots: Option<i32>,
    /// The number of log entries to keep around to sync up slow followers after a snapshot is created.
    #[serde(rename = "LogEntriesForSlowFollowers")]
    log_entries_for_slow_followers: Option<i32>,
    /// The number of ticks that a follower will wait for a message from the leader before becoming a candidate and starting an election. `ElectionTick` must be greater than `HeartbeatTick`.  A tick currently defaults to one second, so these translate directly to seconds currently, but this is NOT guaranteed.
    #[serde(rename = "ElectionTick")]
    election_tick: Option<i32>,
    /// The number of ticks between heartbeats. Every HeartbeatTick ticks, the leader will send a heartbeat to the followers.  A tick currently defaults to one second, so these translate directly to seconds currently, but this is NOT guaranteed.
    #[serde(rename = "HeartbeatTick")]
    heartbeat_tick: Option<i32>,
}

impl SwarmSpecRaft {
    /// Raft configuration.
    pub fn new() -> SwarmSpecRaft {
        SwarmSpecRaft {
            snapshot_interval: None,
            keep_old_snapshots: None,
            log_entries_for_slow_followers: None,
            election_tick: None,
            heartbeat_tick: None,
        }
    }

    pub fn set_snapshot_interval(&mut self, snapshot_interval: i32) {
        self.snapshot_interval = Some(snapshot_interval);
    }

    pub fn with_snapshot_interval(mut self, snapshot_interval: i32) -> SwarmSpecRaft {
        self.snapshot_interval = Some(snapshot_interval);
        self
    }

    pub fn snapshot_interval(&self) -> Option<&i32> {
        self.snapshot_interval.as_ref()
    }

    pub fn reset_snapshot_interval(&mut self) {
        self.snapshot_interval = None;
    }

    pub fn set_keep_old_snapshots(&mut self, keep_old_snapshots: i32) {
        self.keep_old_snapshots = Some(keep_old_snapshots);
    }

    pub fn with_keep_old_snapshots(mut self, keep_old_snapshots: i32) -> SwarmSpecRaft {
        self.keep_old_snapshots = Some(keep_old_snapshots);
        self
    }

    pub fn keep_old_snapshots(&self) -> Option<&i32> {
        self.keep_old_snapshots.as_ref()
    }

    pub fn reset_keep_old_snapshots(&mut self) {
        self.keep_old_snapshots = None;
    }

    pub fn set_log_entries_for_slow_followers(&mut self, log_entries_for_slow_followers: i32) {
        self.log_entries_for_slow_followers = Some(log_entries_for_slow_followers);
    }

    pub fn with_log_entries_for_slow_followers(
        mut self,
        log_entries_for_slow_followers: i32,
    ) -> SwarmSpecRaft {
        self.log_entries_for_slow_followers = Some(log_entries_for_slow_followers);
        self
    }

    pub fn log_entries_for_slow_followers(&self) -> Option<&i32> {
        self.log_entries_for_slow_followers.as_ref()
    }

    pub fn reset_log_entries_for_slow_followers(&mut self) {
        self.log_entries_for_slow_followers = None;
    }

    pub fn set_election_tick(&mut self, election_tick: i32) {
        self.election_tick = Some(election_tick);
    }

    pub fn with_election_tick(mut self, election_tick: i32) -> SwarmSpecRaft {
        self.election_tick = Some(election_tick);
        self
    }

    pub fn election_tick(&self) -> Option<&i32> {
        self.election_tick.as_ref()
    }

    pub fn reset_election_tick(&mut self) {
        self.election_tick = None;
    }

    pub fn set_heartbeat_tick(&mut self, heartbeat_tick: i32) {
        self.heartbeat_tick = Some(heartbeat_tick);
    }

    pub fn with_heartbeat_tick(mut self, heartbeat_tick: i32) -> SwarmSpecRaft {
        self.heartbeat_tick = Some(heartbeat_tick);
        self
    }

    pub fn heartbeat_tick(&self) -> Option<&i32> {
        self.heartbeat_tick.as_ref()
    }

    pub fn reset_heartbeat_tick(&mut self) {
        self.heartbeat_tick = None;
    }
}