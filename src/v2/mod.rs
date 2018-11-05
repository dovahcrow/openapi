//! Support for OpenApi version 2.0 specification.
//!
//! See the
//! [specification](https://github.com/OAI/OpenAPI-Specification/blob/0dd79f6/versions/2.0.md)
//! for more information.

mod schema;

pub use v2::schema::{
    Contact, ExternalDoc, Info, License, Operation, Parameter, ParameterOrRef,
    PathItem, Response, Schema, Scheme, Security, Spec, Tag,
};
