pub mod authentication;
pub mod misc;

use std::cell::RefCell;
use serde::{Serialize, Deserialize};
use reqwest::{Method, blocking::{Client as ReqwestClient, Request, RequestBuilder, Response}};

use self::misc::InfoRequest;

pub trait AuthenticatedBuildRequestBuilder: BuildRequestBuilder {
    fn get(&self, relative: &str) -> RequestBuilder;
    fn post(&self, relative: &str) -> RequestBuilder;
}

pub trait BuildRequestBuilder {
    fn get_anonymous(&self, relative: &str) -> RequestBuilder;
    fn post_anonymous(&self, relative: &str) -> RequestBuilder;
}

pub trait IntoAnonymousRequest {
    fn into_anonymous_request(self, b: &impl BuildRequestBuilder) -> Request;
}

pub trait IntoRequest {
    fn into_request(self, b: &impl AuthenticatedBuildRequestBuilder) -> Request;
}

impl<T: IntoAnonymousRequest> IntoRequest for T {
    fn into_request(self, b: &impl AuthenticatedBuildRequestBuilder) -> Request {
	self.into_anonymous_request(b)
    }
}

pub trait FromResponse {
    type Output;
    fn from_response(response: Response) -> Option<Self::Output>;
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct WithSuccess<T> {
    success: bool,
    #[serde(flatten)]
    body: T
}

pub struct Credentials {
    pub token: String,
    pub user_id: String
}

pub struct Client {
    base: String,
    client: ReqwestClient,
}

pub struct AuthenticatedClient {
    inner: Client,
    credentials: Credentials
}

impl AuthenticatedClient {
    pub fn with_credentials(base: String, credentials: Credentials) -> Self {
	Self {
	    inner: Client::new(base),
	    credentials
	}
    }

    fn from_parts(client: Client, credentials: Credentials) -> Self {
	Self {
	    inner: client,
	    credentials
	}
    }

    pub fn request<T, R>(&self, t: T) -> R
    where T: IntoRequest,
	  T: FromResponse<Output = R> {
	let req = t.into_request(self);
	let resp = self.inner.client.execute(req).unwrap();
	T::from_response(resp).unwrap()
    }
}

impl BuildRequestBuilder for Client {
    fn get_anonymous(&self, relative: &str) -> RequestBuilder {
	self.client.get(format!("{}/{}", self.base, relative))
    }

    fn post_anonymous(&self, relative: &str) -> RequestBuilder {
	self.client.post(format!("{}/{}", self.base, relative))
    }
}

impl BuildRequestBuilder for AuthenticatedClient {
    fn get_anonymous(&self, relative: &str) -> RequestBuilder {
	self.inner.get_anonymous(relative)
    }

    fn post_anonymous(&self, relative: &str) -> RequestBuilder {
	self.inner.post_anonymous(relative)
    }
}

impl AuthenticatedBuildRequestBuilder for AuthenticatedClient {
    fn get(&self, relative: &str) -> RequestBuilder {
	let builder = <Self as BuildRequestBuilder>::get_anonymous(&self, relative);
	builder.header("X-Auth-Token", &self.credentials.token)
	    .header("X-User-Id", &self.credentials.user_id)
    }

    fn post(&self, relative: &str) -> RequestBuilder {
	let builder = <Self as BuildRequestBuilder>::post_anonymous(&self, relative);
	builder.header("X-Auth-Token", &self.credentials.token)
	    .header("X-User-Id", &self.credentials.user_id)
    }
}

impl Client {
    pub fn new(base: String) -> Self {
	Self {
	    base,
	    client: ReqwestClient::new()
	}
    }

    pub fn request<T, R>(&self, t: T) -> R
    where T: IntoAnonymousRequest,
	  T: FromResponse<Output = R> {
	let req = t.into_anonymous_request(self);
	let resp = self.client.execute(req).unwrap();
	T::from_response(resp).unwrap()
    }

    pub fn authenticate(self, auth: authentication::AuthenticationRequest) -> AuthenticatedClient {
	let resp = self.request(auth);
	let credentials = Credentials {
	    token: resp.data.auth_token,
	    user_id: resp.data.user_id,
	};
	AuthenticatedClient::from_parts(self, credentials)
    }
}
