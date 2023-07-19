use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use super::EndPoint;

#[allow(unused_imports)]
use crate::end_point_type::{Get, Post};

/// A trait to link up all the [`EndPoint`] Generics such as [`Get`] and [`Post`].
pub trait EndPointType {
    fn request_builder(client: &Client, url: &str) -> RequestBuilder;
}

/// A marker for any struct that can act as a JSON schema for a URL query.
pub trait QuerySchema: Default + Send + Sync + Serialize + for<'de> Deserialize<'de> {}

/// A marker for any struct that can act as a JSON schema for a POST data.
pub trait PostSchema: Send + Sync + Serialize + for<'de> Deserialize<'de> {}

/// A marker for any struct that can act as a JSON schema for returned JSON data.
pub trait ResponseSchema: Send + Sync + Serialize + for<'de> Deserialize<'de> {}

macro_rules! expand_schemas {
    ($($name:ident),*$(,)?) => {
        /// A marker struct to be used where the [`EndPoint`] does not support a type of schema.
        /// Use this in place of the generic struct.
        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct NotSupported {}
        $(
            impl $name for NotSupported {}
        )*
    };
}

expand_schemas!(QuerySchema, ResponseSchema, PostSchema,);
