use super::{ClientTransformer, RequestTransformer};
use reqwest::{ClientBuilder, RequestBuilder};

pub trait BuildWith<T> {
    fn build_with(self, transformer: &T) -> Self;
}

macro_rules! expand_traits {
    ($(($struct:path, $trait:path)),+$(,)?) => {
        $(
            impl<T> BuildWith<T> for $struct
            where
                T: $trait
            {
                fn build_with(self, transformer: &T) -> Self
                {
                    transformer.transform(self)
                }
            }
        )*

    };
}

expand_traits!(
    (RequestBuilder, RequestTransformer),
    (ClientBuilder, ClientTransformer),
);
