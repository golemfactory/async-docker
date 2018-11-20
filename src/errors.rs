//! Representations of various client errors

use hyper::StatusCode;
use models::ErrorResponse;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        EnvVar(::std::env::VarError);
        Io(::std::io::Error);
        Hyper(::hyper::Error);
        OpenSSL(openssl::error::ErrorStack) #[cfg(feature = "ssl")];
        SerdeJsonError(::serde_json::error::Error);
        InvalidUri(::http::uri::InvalidUri);
        Http(::http::Error);
        Utf8Error(::std::str::Utf8Error);
        FromUtf8Error(::std::string::FromUtf8Error);
        InvalidUriParts(::http::uri::InvalidUriParts);
        InvalidHttpHeaderName(::hyper::header::InvalidHeaderName);
        InvalidHttpHeaderValue(::hyper::header::InvalidHeaderValue);
        StripPrefixError(::std::path::StripPrefixError);
    }

    errors {
        Message(msg: String) {
            description("Message")
                display("{}", msg)
        }

        HyperFault(code: StatusCode) {
            description("HyperFault")
                display("{}", code)
        }

        Utf8 {
            description("Error while trying to handle non-utf8 string")
                display("Error while trying to handle non-utf8 string")
        }

        JsonFieldMissing(name: &'static str) {
            description("JSON Field missing")
                display("JSON Field '{}' missing", name)
        }

        DockerApi(msg: ErrorResponse, status: StatusCode) {
            description("Error response from Docker API")
                display("Docker {} response:\n{:#?}", status.as_str(), msg)
        }

        DockerApiUnknown(msg: String, status: StatusCode) {
            description("Error response from Docker API")
                display("Docker {} response:\n{:#?}", status.as_str(), msg)
        }

        JsonTypeError(fieldname: &'static str, expectedtype: &'static str) {
            description("JSON Field has wrong type")
                display("JSON Field '{}' has wrong type, expected: {}", fieldname, expectedtype)
        }

        NoHostString {
            description("Failed to find a host string")
                display("Failed to find a host string")
        }

        NoPort {
            description("Failed to find a port")
                display("Failed to find a port")
        }

        Eof {
            description("Broken stream")
                display("Broken stream")
        }

        EmptyScheme {
            description("Found empty uri scheme")
                display("Found empty uri scheme")
        }

        InvalidScheme {
            description("Invalid uri scheme")
                display("Invalid uri scheme")
        }

        EmptyPath {
            description("Invalid path - empty parent")
                display("Invalid uri ")
        }
    }
}
