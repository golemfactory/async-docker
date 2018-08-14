use transport::interact::InteractApi;
use std::sync::Arc;
use build::ContainerListOptions;
use futures::Future;
use rep::Container as ContainerRep;
use transport::parse::parse_to_trait;
use Error;
use build::ContainerOptions;
use representation::rep::ContainerCreateInfo;
use communicate::util::build_simple_query;
use hyper::Body;
use transport::interact::InteractApiExt;
use communicate::util::AsSlice;

/// Interface for docker containers
pub struct Containers {
    interact: Arc<InteractApi>,
}

impl Containers {
    /// Exports an interface for interacting with docker containers
    pub(crate) fn new(interact: Arc<InteractApi>) -> Containers {
        Containers {
            interact
        }
    }

    /// Lists the container instances on the docker host
    pub fn list(&self, opts: &ContainerListOptions)
        -> impl Future<Item=Vec<ContainerRep>, Error=Error> {
        let path = "/containers/json";
        let query = opts.serialize();
        let args = (path, query.as_slice());

        parse_to_trait::<Vec<ContainerRep>>(self.interact.get(args))
    }

    /// Returns a builder interface for creating a new container instance
    pub fn create(&self, opts: &ContainerOptions)
                  -> impl Future<Item=ContainerCreateInfo, Error=Error> {
        let path = "/containers/create";
        let query = build_simple_query("name", opts.name.clone());
        let data = opts.serialize().expect("Error during serialization of ContainerOptions");
        let body = Some(Body::from(data));
        let args = (path, query.as_slice(), body);

        parse_to_trait(self.interact.post_json(args))
    }
}