use build::NetworkCreateOptions;
use build::NetworkListOptions;
use communicate::util::AsSlice;
use futures::Future;
use hyper::Body;
use representation::rep::NetworkCreateInfo;
use representation::rep::NetworkDetails;
use std::sync::Arc;
use transport::interact::InteractApi;
use transport::interact::InteractApiExt;
use transport::parse::parse_to_trait;
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
    ) -> impl Future<Item = Vec<NetworkDetails>, Error = Error> {
        let path = "/networks";
        let query = opts.serialize();
        let args = (path, query.as_slice());

        parse_to_trait::<Vec<NetworkDetails>>(self.interact.get(args))
    }

    pub fn create(
        &self,
        opts: &NetworkCreateOptions,
    ) -> impl Future<Item = NetworkCreateInfo, Error = Error> {
        let path = "/networks/create";
        let bytes = opts.serialize().expect("Error during serialization");
        let body = Some(Body::from(bytes));
        let args = (path, body);

        parse_to_trait::<NetworkCreateInfo>(self.interact.post_json(args))
    }
}
