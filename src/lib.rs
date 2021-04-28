pub mod rest {
    #![allow(dead_code)]
    
    use serde::{Serialize, Deserialize};
    use chrono::{DateTime, Utc};
    use reqwest::{Method, blocking::{Client as ReqwestClient, Request, RequestBuilder, Response}};

    pub trait BuildRequestBuilder {
	fn get(&self, relative: &str) -> RequestBuilder;
	fn post(&self, relative: &str) -> RequestBuilder;
    }

    pub trait IntoRequest {
	fn into_request(self, b: &impl BuildRequestBuilder) -> Request;
    }

    pub trait FromResponse {
	type Output;
	fn from_response(response: Response) -> Option<Self::Output>;
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    #[serde(rename_all="camelCase")]
    pub struct BuildInfo {
	node_version: String,
	arch: String,
	platform: String,
	cpus: u32
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    pub struct CommitInfo {
	hash: String,
	date: String,
	author: String,
	subject: String,
	tag: String,
	branch: String
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    pub struct WithSuccess<T> {
	success: bool,
	#[serde(flatten)]
	body: T
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    pub struct InfoResponse {
	info: ServerInfo
    }

    #[derive(Debug)]
    #[derive(Deserialize)]
    pub struct ServerInfo {
	version: String,
	build: Option<BuildInfo>,
	commit: Option<CommitInfo>
    }

    pub struct Credentials {
	pub token: String,
	pub user_id: String
    }

    pub struct Client {
	base: String,
	client: ReqwestClient,
	credentials: Credentials
    }

    impl BuildRequestBuilder for Client {
	fn get(&self, relative: &str) -> RequestBuilder {
	    let builder = self.client.get(format!("{}/{}", self.base, relative));
	    builder.header("X-Auth-Token", &self.credentials.token)
		.header("X-User-Id", &self.credentials.user_id)
	}

	fn post(&self, relative: &str) -> RequestBuilder {
	    let builder = self.client.post(format!("{}/{}", self.base, relative));
	    builder.header("X-Auth-Token", &self.credentials.token)
		.header("X-User-Id", &self.credentials.user_id)
	}
    }

    pub struct InfoRequest;

    impl IntoRequest for InfoRequest {
	fn into_request(self, b: &impl BuildRequestBuilder) -> Request {
	    b.get("/api/info").build().unwrap()
	}
    }

    impl FromResponse for InfoRequest {
	type Output = WithSuccess<InfoResponse>;

	fn from_response(response: Response) -> Option<Self::Output> {
            response.json().ok()
	}
    }

    impl Client {
	pub fn new(base: String, credentials: Credentials) -> Self {
	    Self {
		base,
		credentials,
		client: ReqwestClient::new()
	    }
	}

	pub fn request<T, R>(&self, t: T) -> R
	where T: IntoRequest,
	      T: FromResponse<Output = R> {
	    let req = t.into_request(self);
	    let resp = self.client.execute(req).unwrap();
	    T::from_response(resp).unwrap()
	}
	
    }
}
