//! Interfaces for building various structures
extern crate serde;
extern crate serde_json;

use self::{
    serde::Serialize,
    serde_json::{to_string as ser_to_string, to_value as de_to_value, Value},
};

use std::{
    cmp::Eq,
    collections::{BTreeMap, HashMap},
    hash::Hash,
};
use url::form_urlencoded;

#[derive(Default)]
pub struct PullOptions {
    params: HashMap<&'static str, String>,
}

impl PullOptions {
    /// return a new instance of a builder for options
    pub fn builder() -> PullOptionsBuilder {
        PullOptionsBuilder::new()
    }

    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(form_urlencoded::serialize(&self.params))
        }
    }
}

#[derive(Default)]
pub struct PullOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl PullOptionsBuilder {
    pub fn new() -> PullOptionsBuilder {
        PullOptionsBuilder {
            ..Default::default()
        }
    }

    pub fn image<I>(&mut self, img: I) -> &mut PullOptionsBuilder
    where
        I: Into<String>,
    {
        self.params.insert("fromImage", img.into());
        self
    }

    pub fn src<S>(&mut self, s: S) -> &mut PullOptionsBuilder
    where
        S: Into<String>,
    {
        self.params.insert("fromSrc", s.into());
        self
    }

    pub fn repo<R>(&mut self, r: R) -> &mut PullOptionsBuilder
    where
        R: Into<String>,
    {
        self.params.insert("repo", r.into());
        self
    }

    pub fn tag<T>(&mut self, t: T) -> &mut PullOptionsBuilder
    where
        T: Into<String>,
    {
        self.params.insert("tag", t.into());
        self
    }

    pub fn build(&self) -> PullOptions {
        PullOptions {
            params: self.params.clone(),
        }
    }
}

#[derive(Default)]
pub struct BuildOptions {
    pub path: String,
    params: HashMap<&'static str, String>,
}

impl BuildOptions {
    /// return a new instance of a builder for options
    /// path is expected to be a file path to a directory containing a Dockerfile
    /// describing how to build a Docker image
    pub fn builder<S>(path: S) -> BuildOptionsBuilder
    where
        S: Into<String>,
    {
        BuildOptionsBuilder::new(path)
    }

    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(form_urlencoded::serialize(&self.params))
        }
    }
}

#[derive(Default)]
pub struct BuildOptionsBuilder {
    path: String,
    params: HashMap<&'static str, String>,
}

impl BuildOptionsBuilder {
    /// path is expected to be a file path to a directory containing a Dockerfile
    /// describing how to build a Docker image
    pub fn new<S>(path: S) -> BuildOptionsBuilder
    where
        S: Into<String>,
    {
        BuildOptionsBuilder {
            path: path.into(),
            ..Default::default()
        }
    }

    /// set the name of the docker file. defaults to "DockerFile"
    pub fn dockerfile<P>(&mut self, path: P) -> &mut BuildOptionsBuilder
    where
        P: Into<String>,
    {
        self.params.insert("dockerfile", path.into());
        self
    }

    /// tag this image with a name after building it
    pub fn tag<T>(&mut self, t: T) -> &mut BuildOptionsBuilder
    where
        T: Into<String>,
    {
        self.params.insert("t", t.into());
        self
    }

    pub fn remote<R>(&mut self, r: R) -> &mut BuildOptionsBuilder
    where
        R: Into<String>,
    {
        self.params.insert("remote", r.into());
        self
    }

    /// don't use the image cache when building image
    pub fn nocache<R>(&mut self, nc: bool) -> &mut BuildOptionsBuilder {
        self.params.insert("nocache", nc.to_string());
        self
    }

    pub fn rm(&mut self, r: bool) -> &mut BuildOptionsBuilder {
        self.params.insert("rm", r.to_string());
        self
    }

    pub fn forcerm(&mut self, fr: bool) -> &mut BuildOptionsBuilder {
        self.params.insert("forcerm", fr.to_string());
        self
    }

    /// `bridge`, `host`, `none`, `container:<name|id>`, or a custom network name.
    pub fn network_mode<T>(&mut self, t: T) -> &mut BuildOptionsBuilder
    where
        T: Into<String>,
    {
        self.params.insert("networkmode", t.into());
        self
    }

    // todo: memory
    // todo: memswap
    // todo: cpushares
    // todo: cpusetcpus
    // todo: cpuperiod
    // todo: cpuquota
    // todo: buildargs

    pub fn build(&self) -> BuildOptions {
        BuildOptions {
            path: self.path.clone(),
            params: self.params.clone(),
        }
    }
}

/// Options for filtering container list results
#[derive(Default)]
pub struct ContainerListOptions {
    params: HashMap<&'static str, String>,
}

impl ContainerListOptions {
    /// return a new instance of a builder for options
    pub fn builder() -> ContainerListOptionsBuilder {
        ContainerListOptionsBuilder::new()
    }

    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(form_urlencoded::serialize(&self.params))
        }
    }
}

/// Filter options for container listings
pub enum ContainerFilter {
    ExitCode(u64),
    Status(String),
    LabelName(String),
    Label(String, String),
}

/// Builder interface for `ContainerListOptions`
#[derive(Default)]
pub struct ContainerListOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl ContainerListOptionsBuilder {
    pub fn new() -> ContainerListOptionsBuilder {
        ContainerListOptionsBuilder {
            ..Default::default()
        }
    }

    pub fn filter(&mut self, filters: Vec<ContainerFilter>) -> &mut ContainerListOptionsBuilder {
        let mut param = HashMap::new();

        for f in filters {
            match f {
                ContainerFilter::ExitCode(c) => param.insert("exit", vec![c.to_string()]),
                ContainerFilter::Status(s) => param.insert("status", vec![s]),
                ContainerFilter::LabelName(n) => param.insert("label", vec![n]),
                ContainerFilter::Label(n, v) => param.insert("label", vec![format!("{}={}", n, v)]),
            };
        }

        // structure is a a json encoded object mapping string keys to a list
        // of string values
        self.params.insert(
            "filters",
            ser_to_string(&param).expect("Filter args serialization failed"),
        );
        self
    }

    pub fn all(&mut self) -> &mut ContainerListOptionsBuilder {
        self.params.insert("all", "true".to_owned());
        self
    }

    pub fn since(&mut self, since: &str) -> &mut ContainerListOptionsBuilder {
        self.params.insert("since", since.to_owned());
        self
    }

    pub fn before(&mut self, before: &str) -> &mut ContainerListOptionsBuilder {
        self.params.insert("before", before.to_owned());
        self
    }

    pub fn sized(&mut self) -> &mut ContainerListOptionsBuilder {
        self.params.insert("size", "true".to_owned());
        self
    }

    pub fn build(&self) -> ContainerListOptions {
        ContainerListOptions {
            params: self.params.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct ContainerArchivePutOptions {
    #[serde(skip)]
    pub local_path: String,
    #[serde(flatten)]
    params: HashMap<&'static str, String>,
    #[serde(flatten)]
    params_bool: HashMap<&'static str, bool>,
}

impl ContainerArchivePutOptions {
    /// return a new instance of a builder for options
    pub fn builder() -> ContainerArchiveOptionsBuilder {
        ContainerArchiveOptionsBuilder::new()
    }

    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(form_urlencoded::serialize(&self.params))
        }
    }
}

#[derive(Default)]
pub struct ContainerArchiveOptionsBuilder {
    local_path: String,
    params: HashMap<&'static str, String>,
    params_bool: HashMap<&'static str, bool>,
}

impl ContainerArchiveOptionsBuilder {
    pub fn new() -> ContainerArchiveOptionsBuilder {
        ContainerArchiveOptionsBuilder {
            local_path: String::new(),
            params: HashMap::new(),
            params_bool: HashMap::new(),
        }
    }

    pub fn remote_path(&mut self, cmds: String) -> &mut ContainerArchiveOptionsBuilder {
        self.params.insert("path", cmds);
        self
    }

    pub fn local_path(&mut self, path: String) -> &mut ContainerArchiveOptionsBuilder {
        self.local_path = path;
        self
    }

    pub fn no_overwrite(&mut self, o: bool) -> &mut ContainerArchiveOptionsBuilder {
        self.params_bool.insert("noOverwriteDirNonDir", o);
        self
    }

    pub fn build(&self) -> ContainerArchivePutOptions {
        ContainerArchivePutOptions {
            local_path: self.local_path.clone(),
            params: self.params.clone(),
            params_bool: self.params_bool.clone(),
        }
    }
}

/// Options for filtering streams of Docker events
#[derive(Default)]
pub struct EventsOptions {
    params: HashMap<&'static str, String>,
}

impl EventsOptions {
    pub fn builder() -> EventsOptionsBuilder {
        EventsOptionsBuilder::new()
    }

    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(form_urlencoded::serialize(&self.params))
        }
    }
}

pub enum EventFilterType {
    Container,
    Image,
    Volume,
    Network,
    Daemon,
}

fn event_filter_type_to_string(filter: EventFilterType) -> &'static str {
    match filter {
        EventFilterType::Container => "container",
        EventFilterType::Image => "image",
        EventFilterType::Volume => "volume",
        EventFilterType::Network => "network",
        EventFilterType::Daemon => "daemon",
    }
}

/// Filter options for image listings
pub enum EventFilter {
    Container(String),
    Event(String),
    Image(String),
    Label(String),
    Type(EventFilterType),
    Volume(String),
    Network(String),
    Daemon(String),
}

/// Builder interface for `EventOptions`
#[derive(Default)]
pub struct EventsOptionsBuilder {
    params: HashMap<&'static str, String>,
    events: Vec<String>,
    containers: Vec<String>,
    images: Vec<String>,
    labels: Vec<String>,
    volumes: Vec<String>,
    networks: Vec<String>,
    daemons: Vec<String>,
    types: Vec<String>,
}

impl EventsOptionsBuilder {
    pub fn new() -> EventsOptionsBuilder {
        EventsOptionsBuilder {
            ..Default::default()
        }
    }

    /// Filter events since a given timestamp
    pub fn since(&mut self, ts: &u64) -> &mut EventsOptionsBuilder {
        self.params.insert("since", ts.to_string());
        self
    }

    /// Filter events until a given timestamp
    pub fn until(&mut self, ts: &u64) -> &mut EventsOptionsBuilder {
        self.params.insert("until", ts.to_string());
        self
    }

    pub fn filter(&mut self, filters: Vec<EventFilter>) -> &mut EventsOptionsBuilder {
        let mut params = HashMap::new();
        for f in filters {
            match f {
                EventFilter::Container(n) => {
                    self.containers.push(n);
                    params.insert("container", self.containers.clone())
                }
                EventFilter::Event(n) => {
                    self.events.push(n);
                    params.insert("event", self.events.clone())
                }
                EventFilter::Image(n) => {
                    self.images.push(n);
                    params.insert("image", self.images.clone())
                }
                EventFilter::Label(n) => {
                    self.labels.push(n);
                    params.insert("label", self.labels.clone())
                }
                EventFilter::Volume(n) => {
                    self.volumes.push(n);
                    params.insert("volume", self.volumes.clone())
                }
                EventFilter::Network(n) => {
                    self.networks.push(n);
                    params.insert("network", self.networks.clone())
                }
                EventFilter::Daemon(n) => {
                    self.daemons.push(n);
                    params.insert("daemon", self.daemons.clone())
                }
                EventFilter::Type(n) => {
                    let event_type = event_filter_type_to_string(n).to_string();
                    self.types.push(event_type);
                    params.insert("type", self.types.clone())
                }
            };
        }
        self.params.insert(
            "filters",
            ser_to_string(&params).expect("Error during serialization"),
        );
        self
    }

    pub fn build(&self) -> EventsOptions {
        EventsOptions {
            params: self.params.clone(),
        }
    }
}

/// Options for controlling log request results
#[derive(Default)]
pub struct LogsOptions {
    params: HashMap<&'static str, String>,
}

impl LogsOptions {
    /// return a new instance of a builder for options
    pub fn builder() -> LogsOptionsBuilder {
        LogsOptionsBuilder::new()
    }

    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(form_urlencoded::serialize(&self.params))
        }
    }
}

/// Builder interface for `LogsOptions`
#[derive(Default)]
pub struct LogsOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl LogsOptionsBuilder {
    pub fn new() -> LogsOptionsBuilder {
        LogsOptionsBuilder {
            ..Default::default()
        }
    }

    pub fn follow(&mut self, f: bool) -> &mut LogsOptionsBuilder {
        self.params.insert("follow", f.to_string());
        self
    }

    pub fn stdout(&mut self, s: bool) -> &mut LogsOptionsBuilder {
        self.params.insert("stdout", s.to_string());
        self
    }

    pub fn stderr(&mut self, s: bool) -> &mut LogsOptionsBuilder {
        self.params.insert("stderr", s.to_string());
        self
    }

    pub fn timestamps(&mut self, t: bool) -> &mut LogsOptionsBuilder {
        self.params.insert("timestamps", t.to_string());
        self
    }

    /// how_many can either by "all" or a to_string() of the number
    pub fn tail(&mut self, how_many: &str) -> &mut LogsOptionsBuilder {
        self.params.insert("tail", how_many.to_owned());
        self
    }

    pub fn build(&self) -> LogsOptions {
        LogsOptions {
            params: self.params.clone(),
        }
    }
}

/// Filter options for image listings
pub enum ImageFilter {
    Dangling,
    LabelName(String),
    Label(String, String),
}

/// Options for filtering image list results
#[derive(Default)]
pub struct ImageListOptions {
    params: HashMap<&'static str, String>,
}

impl ImageListOptions {
    pub fn builder() -> ImageListOptionsBuilder {
        ImageListOptionsBuilder::new()
    }
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(form_urlencoded::serialize(&self.params))
        }
    }
}

/// Builder interface for `ImageListOptions`
#[derive(Default)]
pub struct ImageListOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl ImageListOptionsBuilder {
    pub fn new() -> ImageListOptionsBuilder {
        ImageListOptionsBuilder {
            ..Default::default()
        }
    }

    pub fn digests(&mut self, d: bool) -> &mut ImageListOptionsBuilder {
        self.params.insert("digests", d.to_string());
        self
    }

    pub fn all(&mut self, a: bool) -> &mut ImageListOptionsBuilder {
        self.params.insert("all", a.to_string());
        self
    }

    pub fn filter_name(&mut self, name: &str) -> &mut ImageListOptionsBuilder {
        self.params.insert("filter", name.to_owned());
        self
    }

    pub fn filter(&mut self, filters: Vec<ImageFilter>) -> &mut ImageListOptionsBuilder {
        let mut param = HashMap::new();
        for f in filters {
            match f {
                ImageFilter::Dangling => param.insert("dangling", vec![true.to_string()]),
                ImageFilter::LabelName(n) => param.insert("label", vec![n]),
                ImageFilter::Label(n, v) => param.insert("label", vec![format!("{}={}", n, v)]),
            };
        }
        // structure is a a json encoded object mapping string keys to a list
        // of string values
        self.params.insert(
            "filters",
            ser_to_string(&param).expect("Error during deserialization"),
        );
        self
    }

    pub fn build(&self) -> ImageListOptions {
        ImageListOptions {
            params: self.params.clone(),
        }
    }
}

/// Options for controlling log request results
#[derive(Default)]
pub struct RmContainerOptions {
    params: HashMap<&'static str, String>,
}

impl RmContainerOptions {
    /// return a new instance of a builder for options
    pub fn builder() -> RmContainerOptionsBuilder {
        RmContainerOptionsBuilder::new()
    }

    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(form_urlencoded::serialize(&self.params))
        }
    }
}

/// Builder interface for `LogsOptions`
#[derive(Default)]
pub struct RmContainerOptionsBuilder {
    params: HashMap<&'static str, String>,
}

impl RmContainerOptionsBuilder {
    pub fn new() -> RmContainerOptionsBuilder {
        RmContainerOptionsBuilder {
            ..Default::default()
        }
    }

    pub fn force(&mut self, f: bool) -> &mut RmContainerOptionsBuilder {
        self.params.insert("force", f.to_string());
        self
    }

    pub fn volumes(&mut self, s: bool) -> &mut RmContainerOptionsBuilder {
        self.params.insert("v", s.to_string());
        self
    }

    pub fn build(&self) -> RmContainerOptions {
        RmContainerOptions {
            params: self.params.clone(),
        }
    }
}

/// Options for filtering networks list results
#[derive(Default)]
pub struct NetworkListOptions {
    params: HashMap<&'static str, String>,
}

impl NetworkListOptions {
    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(form_urlencoded::serialize(&self.params))
        }
    }
}

/// Interface for connect container to network
#[derive(Serialize)]
pub struct ContainerConnectionOptions {
    pub container: Option<String>,
    #[serde(flatten)]
    params: HashMap<&'static str, String>,
}

impl ContainerConnectionOptions {
    pub fn new(container_id: &str) -> ContainerConnectionOptions {
        let mut params = HashMap::new();
        params.insert("Container", container_id.to_owned());
        ContainerConnectionOptions {
            container: None,
            params: params.clone(),
        }
    }

    /// serialize options as a string. returns None if no options are defined
    pub fn serialize(&self) -> Option<String> {
        Some(ser_to_string(&self).expect("ContainerConnectionOptions serialization failed"))
    }

    pub fn parse_from<'a, K, V>(
        &self,
        params: &'a HashMap<K, V>,
        body: &mut BTreeMap<String, Value>,
    ) where
        K: ToString + Eq + Hash,
        V: Serialize,
    {
        for (k, v) in params.iter() {
            let key = k.to_string();
            let value = de_to_value(v).expect("Error during deserialization");

            body.insert(key, value);
        }
    }

    pub fn force(&mut self) -> ContainerConnectionOptions {
        self.params.insert("Force", "true".to_owned());
        ContainerConnectionOptions {
            container: None,
            params: self.params.clone(),
        }
    }
}
