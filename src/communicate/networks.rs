use build::NetworkListOptions;
use communicate::util::AsSlice;
use futures::Future;
use hyper::Body;
use models::Network;
use models::NetworkConfig;
use models::NetworkCreateResponse;
use std::sync::Arc;
use transport::{
    interact::{InteractApi, InteractApiExt},
    parse::parse_to_trait,
};
use Error;

/// Interface for docker networks
pub struct Networks {
    interact: Arc<InteractApi>,
}

impl Networks {
    /// Exports an interface for interacting with docker Networks
    pub(crate) fn new(interact: Arc<InteractApi>) -> Networks {
        Networks { interact }
    }

    /// List the docker networks on the current docker host
    pub fn list(
        &self,
        opts: &NetworkListOptions,
    ) -> impl Future<Item = Vec<Network>, Error = Error> {
        let path = "/networks";
        let query = opts.serialize();
        let args = (path, query.as_slice());

        parse_to_trait::<Vec<Network>>(self.interact.get(args))
    }

    pub fn create(
        &self,
        opts: &NetworkConfig,
    ) -> impl Future<Item = NetworkCreateResponse, Error = Error> {
        let path = "/networks/create";
        let body = serde_json::ser::to_string(opts).map(|s| Body::from(s)).ok();
        let args = (path, body);

        parse_to_trait::<NetworkCreateResponse>(self.interact.post_json(args))
    }
}
