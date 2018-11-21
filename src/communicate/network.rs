use build::ContainerConnectionOptions;
use communicate::util::AsSlice;
use futures::Future;
use http::StatusCode;
use models;
use std::{borrow::Cow, sync::Arc};
use transport::{
    interact::{InteractApi, InteractApiExt},
    parse::{parse_to_trait, status_code},
};
use Error;

/// Interface for accessing and manipulating a docker network
pub struct Network<'b> {
    interact: Arc<InteractApi>,
    id: Cow<'b, str>,
}

impl<'b> Network<'b> {
    /// Exports an interface exposing operations against a network instance
    pub(crate) fn new<S>(interact: Arc<InteractApi>, id: S) -> Network<'b>
    where
        S: Into<Cow<'b, str>>,
    {
        Network {
            interact,
            id: id.into(),
        }
    }

    /// a getter for the Network id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Inspects the current docker network instance's details
    pub fn inspect(&self) -> impl Future<Item = models::Network, Error = Error> {
        let path = format!("/networks/{}", self.id);

        parse_to_trait::<models::Network>(self.interact.get(path.as_str()))
    }

    /// Delete the network instance
    pub fn delete(&self) -> impl Future<Item = StatusCode, Error = Error> {
        let path = format!("/networks/{}", self.id);

        status_code(self.interact.delete(path.as_str()))
    }

    /// Connect container to network
    pub fn connect(
        &self,
        opts: &ContainerConnectionOptions,
    ) -> impl Future<Item = StatusCode, Error = Error> {
        let path = format!("/networks/{}/connect", self.id);
        let query = opts.serialize();
        let args = (path.as_str(), query.as_slice());

        status_code(self.interact.post(args))
    }

    /// Disconnect container to network
    pub fn disconnect(
        &self,
        opts: &ContainerConnectionOptions,
    ) -> impl Future<Item = StatusCode, Error = Error> {
        let path = format!("/networks/{}/disconnect", self.id);
        let query = opts.serialize();
        let args = (path.as_str(), query.as_slice());

        status_code(self.interact.post(args))
    }
}
