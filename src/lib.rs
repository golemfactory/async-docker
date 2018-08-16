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


extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate futures;
extern crate hyper;
extern crate http;
extern crate url;
extern crate bytes;
extern crate tokio_codec;

pub mod representation;
pub mod communicate;
pub mod build;

mod errors;
mod tarball;
mod transport;


pub use errors::Error;
pub use errors::Result;

pub use communicate::*;
pub use build::*;
pub use representation::*;