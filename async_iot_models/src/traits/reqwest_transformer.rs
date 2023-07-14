macro_rules! expand_traits {
    ($(($struct:ident, $trait:ident)),+$(,)?) => {
        $(
            use reqwest::$struct;

            pub trait $trait {
                fn transform(&self, builder: $struct) -> $struct;
            }

            impl<T> $trait for Option<T>
            where
                T: $trait
            {
                fn transform(&self, builder: $struct) -> $struct {
                    match self.as_ref() {
                        Some(transformer) => transformer.transform(builder),
                        None => builder
                    }
                }
            }
        )*
    };
}

expand_traits!(
    (RequestBuilder, RequestTransformer),
    (ClientBuilder, ClientTransformer),
);
