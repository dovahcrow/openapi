//! Support for OpenApi version 3.0.1 specification.
//!
//! See the
//! [specification](https://github.com/OAI/OpenAPI-Specification/blob/0dd79f6/versions/3.0.1.md)
//! for more information.

mod components;
mod schema;

pub use v3_0::components::{Components, ObjectOrReference};
pub use v3_0::schema::{
    Callback, Contact, Encoding, Example, ExternalDoc, Header, Info, License, Link, MediaType, MediaTypeExample, Operation, Parameter, PathItem, RequestBody, Response, Schema,
    SecurityScheme, Server, Spec, Tag, Url,
};
