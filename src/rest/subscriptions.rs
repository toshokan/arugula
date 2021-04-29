use serde::{Serialize, Deserialize};

use reqwest::blocking::{Request, Response};
use super::{AuthenticatedBuildRequestBuilder, Changeset, FromResponse, IntoRequest, WithSuccess};

#[derive(Debug)]
#[derive(Deserialize)]
pub struct UserDataShort {
    #[serde(rename="_id")]
    id: String,
    username: String,
    name: Option<String>
}

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct SubscriptionData {
    t: String,
    ts: Option<String>,
    name: Option<String>,
    fname: Option<String>,
    rid: String,
    u: UserDataShort,
    open: bool,
    alert: bool,
    unread: u32,
    user_mentions: u32,
    group_mentions: u32,
    #[serde(rename="_updatedAt")]
    updated_at: String,
    #[serde(rename="_id")]
    id: String
}

pub struct GetRequest;

impl IntoRequest for GetRequest {
    fn into_request(self, b: &impl AuthenticatedBuildRequestBuilder) -> Request {
	b.get("api/v1/subscriptions.get").build().unwrap()
    }
}

impl FromResponse for GetRequest {
    type Output = WithSuccess<Changeset<SubscriptionData>>;

    fn from_response(response: Response) -> Option<Self::Output> {
	response.json().unwrap()
    }
}

#[derive(Debug)]
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct GetOneRequest {
    pub room_id: String
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct GetOneResponse {
    subscription: Option<SubscriptionData>
}

impl IntoRequest for GetOneRequest {
    fn into_request(self, b: &impl AuthenticatedBuildRequestBuilder) -> Request {
	b.get("api/v1/subscriptions.getOne").query(&self).build().unwrap()
    }
}

impl FromResponse for GetOneRequest {
    type Output = WithSuccess<GetOneResponse>;

    fn from_response(response: Response) -> Option<Self::Output> {
	response.json().unwrap()
    }
}
