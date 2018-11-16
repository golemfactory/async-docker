//! Shiplift is a multi-transport utility for maneuvering [docker](https://www.interact.com/) containers
//!
//! # examples
//!
//! ```no_run
//! extern crate async_docker;
//!
//! let docker = async_docker::Docker::new();
//! let images = docker.images().list(&Default::default()).unwrap();
//! debug!("docker images in stock");
//! for i in images {
//!   debug!("{:?}", i.RepoTags);
//! }
//! ```

#![recursion_limit = "256"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

extern crate bytes;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate tokio_codec;
extern crate url;

pub mod build;
pub mod communicate;
pub mod representation;

mod errors;
mod tarball;
mod transport;

pub use errors::Error;
pub use errors::Result;

pub use build::*;
pub use communicate::*;
pub use representation::*;
