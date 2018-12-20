use build::{BuildOptions, ImageListOptions, PullOptions};
use communicate::util::{build_simple_query, AsSlice};
use futures::{future, Future, Stream};
use hyper::Body;
use models::ContainerTopResponse;
use models::Image;
use models::ImageSearchResponseItem;
use serde_json::Value;
use std::sync::Arc;
use tarball::tarball;
use transport::{
    interact::{InteractApi, InteractApiExt},
    parse::{parse_to_lines, parse_to_trait},
};
use url::form_urlencoded;
use Error;
use transport::parse::empty_result2;
use http::StatusCode;

/// Interface for docker images
pub struct Images {
    interact: Arc<InteractApi>,
}

impl Images {
    /// Exports an interface for interacting with docker images
    pub(crate) fn new(interact: Arc<InteractApi>) -> Images {
        Images { interact }
    }

    /// Builds a new image build by reading a Dockerfile in a target directory
    pub fn build(
        &self,
        opts: &BuildOptions,
    ) -> impl Future<Item = Vec<ContainerTopResponse>, Error = Error> + Send {
        let mut bytes = vec![];
        let interact = self.interact.clone();

        let path = "/build";
        let query = opts.serialize();

        future::result(tarball::dir(&mut bytes, &opts.path[..])).and_then(move |_| {
            let body = Some(Body::from(bytes));

            let args = (path, query.as_slice(), body);
            parse_to_trait::<Vec<ContainerTopResponse>>(interact.get(args))
        })
    }

    /// Lists the docker images on the current docker host
    pub fn list(
        &self,
        opts: &ImageListOptions,
    ) -> impl Future<Item = Vec<Image>, Error = Error> + Send {
        let path = "/images/json";
        let query = opts.serialize();

        let args = (path, query.as_slice());

        parse_to_trait::<Vec<Image>>(self.interact.get(args))
    }

    /// Search for docker images by term
    pub fn search(
        &self,
        term: &str,
    ) -> impl Future<Item = Vec<ImageSearchResponseItem>, Error = Error> + Send {
        let path = "/images/search";
        let query = build_simple_query("term", Some(term));

        let args = (path, query.as_slice());

        parse_to_trait::<Vec<ImageSearchResponseItem>>(self.interact.get(args))
    }

    /// Pull and create a new docker images from an existing image
    pub fn pull(&self, opts: &PullOptions) -> impl Future<Item = StatusCode, Error = Error> + Send {
        let path = "/images/create";
        let query = opts.serialize();

        let args = (path, query.as_slice());

        empty_result2(self.interact.post(args))
    }

    /// exports a collection of named images,
    /// either by name, name:tag, or image id, into a tarball
    pub fn export(&self, names: Vec<&str>) -> impl Stream<Item = String, Error = Error> + Send {
        let params = names
            .iter()
            .map(|n| ("names", *n))
            .collect::<Vec<(&str, &str)>>();

        let path = "/images/get";
        let query = Some(form_urlencoded::serialize(params));
        let args = (path, query.as_slice());

        parse_to_lines(self.interact.get(args))
    }
}
