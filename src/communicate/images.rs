use std::sync::Arc;
use transport::interact::InteractApi;
use build::BuildOptions;
use futures::Stream;
use representation::rep::Top;
use {Result, Error};
use tarball::tarball;
use build::ImageListOptions;
use representation::rep::SearchResult;
use serde_json::Value;
use url::form_urlencoded;
use build::PullOptions;
use hyper::Body;
use transport::parse::parse_to_stream;
use transport::parse::parse_to_trait;
use communicate::util::AsSlice;
use transport::interact::InteractApiExt;
use futures::future;
use futures::Future;
use rep::Image as ImageRep;
use communicate::util::build_simple_query;
use transport::parse::parse_to_lines;

/// Interface for docker images
pub struct Images
{
    interact: Arc<InteractApi>,
}

impl Images {
    /// Exports an interface for interacting with docker images
    pub(crate) fn new(interact: Arc<InteractApi>) -> Images
    {
        Images {
            interact,
        }
    }

    /// Builds a new image build by reading a Dockerfile in a target directory
    pub fn build(&self, opts: &BuildOptions) -> impl Future<Item=Vec<Top>, Error=Error> + Send {
        let mut bytes = vec![];
        let interact = self.interact.clone();

        let path = "/build";
        let query = opts.serialize();

        future::result(tarball::dir(&mut bytes, &opts.path[..]))
            .and_then(move |_| {
                let body = Some(Body::from(bytes));

                let args = (path, query.as_slice(), body);
                parse_to_trait::<Vec<Top>>(interact.get(args))
            })
    }

    /// Lists the docker images on the current docker host
    pub fn list(&self, opts: &ImageListOptions) -> impl Future<Item=Vec<ImageRep>, Error=Error> + Send {
        let path = "/images/json";
        let query = opts.serialize();

        let args = (path, query.as_slice());

        parse_to_trait::<Vec<ImageRep>>(self.interact.get(args))
    }


    /// Search for docker images by term
    pub fn search(&self, term: &str) -> impl Future<Item=Vec<SearchResult>, Error=Error> + Send {
        let path = "/images/search";
        let query = build_simple_query("term", Some(term));

        let args = (path, query.as_slice());

        parse_to_trait::<Vec<SearchResult>>(self.interact.get(args))
    }

    /// Pull and create a new docker images from an existing image
    pub fn pull(&self, opts: &PullOptions) -> impl Future<Item=Value, Error=Error> + Send {
        let path = "/images/create";
        let query = opts.serialize();

        let args = (path, query.as_slice());

        parse_to_trait::<Value>(self.interact.post(args))
    }

    /// exports a collection of named images,
    /// either by name, name:tag, or image id, into a tarball
    pub fn export(&self, names: Vec<&str>) -> impl Stream<Item=String, Error=Error> + Send {
        let params = names
            .iter()
            .map(|n| ("names", *n))
            .collect::<Vec<(&str, &str)>>();

        let path = "/images/get";
        let query = Some(form_urlencoded::serialize(params));
        let args = (path, query.as_slice());

        parse_to_lines(self.interact.get(args))
    }

    // pub fn import(self, tarball: Read>) -> Result<()> {
    //  self.interact.post
    // }
}