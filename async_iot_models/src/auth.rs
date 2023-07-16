use serde::{Deserialize, Serialize};

use crate::traits::RequestTransformer;

#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize)]
pub struct BasicAuthentication {
    username: String,
    password: Option<String>,
}

impl BasicAuthentication {
    pub fn new<U, P>(username: U, password: Option<P>) -> Self
    where
        U: ToString,
        P: ToString,
    {
        Self {
            username: username.to_string(),
            password: password.map(|p| p.to_string()),
        }
    }
}

impl RequestTransformer for BasicAuthentication {
    fn transform(&self, request_builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request_builder.basic_auth(&self.username, self.password.as_ref())
    }
}

#[allow(dead_code)]
#[derive(Clone, Serialize, Deserialize)]
pub struct TokenAuthentication {
    token: String,
}

impl RequestTransformer for TokenAuthentication {
    fn transform(&self, request_builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request_builder.bearer_auth(&self.token)
    }
}
