use communicate::docker::Docker;
use representation::rep::ImageDetails;
use representation::rep::History;
use Error;
use Result;
use representation::rep::Status;
use serde_json::Value;
use std::borrow::Cow;
use errors::ErrorKind as EK;
use futures::Stream;
use transport::parse::parse_to_trait;
use futures::Future;
use transport::parse::parse_to_file;
use hyper::client::connect::Connect;
use std::option::Iter;
use futures::future;
use transport::interact::Interact;
use std::sync::Arc;
use transport::interact::InteractApi;
use transport::interact::InteractApiExt;
use hyper::Chunk;


/// Interface for accessing and manipulating a named docker image
pub struct Image<'b>
{
    interact: Arc<InteractApi>,
    name: Cow<'b, str>,
}

impl<'b> Image<'b>
{
    /// Exports an interface for operations that may be performed against a named image
    pub(crate) fn new<S>(interact: Arc<InteractApi>, name: S) -> Image<'b>
    where
        S: Into<Cow<'b, str>>,
    {
        Image {
            interact,
            name: name.into(),
        }
    }

    /// Inspects a named image's details
    pub fn inspect(&self) -> impl Future<Item=ImageDetails, Error=Error> + Send {
        let args = format!("/images/{}/json", self.name);

        parse_to_trait::<ImageDetails>(self.interact.get(args.as_str()))
    }

    /// Lists the history of the images set of changes
    pub fn history(&self) -> impl Future<Item=History, Error=Error> + Send {
        let args = format!("/images/{}/history", self.name);

        parse_to_trait::<History>(self.interact.get(args.as_str()))
    }

    /// Deletes an image
    pub fn delete(&self) -> impl Future<Item=Vec<Status>, Error=Error> + Send {

        fn parse_array(xs: Vec<Value>) -> Result<Vec<Status>> {
            xs
                .iter()
                .map(|j| {
                    let obj = j
                        .as_object()
                        .ok_or_else(|| Error::from(EK::JsonTypeError("<anonymous>", "Object")))?;


                    if let Some(sha) = obj.get("Untagged") {
                        sha.as_str()
                            .map(|s| Status::Untagged(s.to_owned()))
                            .ok_or_else(|| Error::from(EK::JsonTypeError("Untagged", "String")))
                    } else {
                        obj.get("Deleted")
                            .ok_or_else(|| Error::from(EK::JsonFieldMissing("Deleted' or 'Untagged")))
                            .and_then(|sha| {
                                sha.as_str()
                                    .map(|s| Status::Deleted(s.to_owned()))
                                    .ok_or_else(|| Error::from(EK::JsonTypeError("Deleted", "String")))
                            })
                    }
                })
                .collect()
        }

        let args = format!("/images/{}", self.name);

        parse_to_trait::<Value>(self.interact.delete(args.as_str()))
            .and_then(|val|
                match val {
                    Value::Array(xs) => future::result(parse_array(xs)),
                    _ => unreachable!(),
                }
            )
    }

    /// Export this image to a tarball
    pub fn export(&self) -> impl Stream<Item=Chunk, Error=Error> + Send {
        let path = format!("/images/{}/get", self.name);

        self.interact.get(path.as_str())
            .and_then(|a| a.map_err(Error::from))
            .and_then(|a| Ok(a.into_body().map_err(Error::from)))
            .flatten_stream()
    }
}