use reqwest::blocking::{Request, Response};
use serde::Deserialize;

use crate::rest::{AuthenticatedBuildRequestBuilder, FromResponse, IntoRequest, WithSuccess};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    node_version: String,
    arch: String,
    platform: String,
    cpus: u32,
}

#[derive(Debug, Deserialize)]
pub struct CommitInfo {
    hash: String,
    date: String,
    author: String,
    subject: String,
    tag: String,
    branch: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerInfo {
    version: String,
    build: Option<BuildInfo>,
    commit: Option<CommitInfo>,
}

pub struct InfoRequest;

#[derive(Debug, Deserialize)]
pub struct InfoResponse {
    info: ServerInfo,
}

impl IntoRequest for InfoRequest {
    fn into_request(self, b: &impl AuthenticatedBuildRequestBuilder) -> Request {
        b.get("/api/info").build().unwrap()
    }
}

impl FromResponse for InfoRequest {
    type Output = WithSuccess<InfoResponse>;

    fn from_response(response: Response) -> Option<Self::Output> {
        response.json().ok()
    }
}
