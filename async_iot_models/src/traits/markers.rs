use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

pub trait EndPointType {
    fn request_builder(client: &Client, url: &str) -> RequestBuilder;
}
pub trait QuerySchema: Default + Send + Sync + Serialize + for<'de> Deserialize<'de> {}
pub trait PostSchema: Send + Sync + Serialize + for<'de> Deserialize<'de> {}
pub trait ResponseSchema: Send + Sync + Serialize + for<'de> Deserialize<'de> {}

macro_rules! expand_schemas {
    ($($name:ident),*$(,)?) => {
        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct NotSupported {}
        $(
            impl $name for NotSupported {}
        )*
    };
}

expand_schemas!(QuerySchema, ResponseSchema, PostSchema,);
