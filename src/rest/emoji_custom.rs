use serde::{Serialize, Deserialize};

use reqwest::blocking::{Request, Response};
use super::{AuthenticatedBuildRequestBuilder, Changeset, FromResponse, IntoRequest, WithSuccess};

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct EmojiData {
    #[serde(rename="_id")]
    id: String,
    name: String,
    aliases: Vec<String>,
    extension: String,
    #[serde(rename="_updatedAt")]
    updated_at: String
}

pub struct NewEmoji {
    emoji: Vec<u8>,
    name: String,
    aliases: Vec<String>
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct EmojiResponseData {
    emojis: Changeset<EmojiData>
}

pub struct ListRequest;

impl IntoRequest for ListRequest {
    fn into_request(self, b: &impl AuthenticatedBuildRequestBuilder) -> Request {
	b.get("api/v1/emoji-custom.list").build().unwrap()
    }
}

impl FromResponse for ListRequest {
    type Output = WithSuccess<EmojiResponseData>;

    fn from_response(response: Response) -> Option<Self::Output> {
	response.json().unwrap()
    }
}

#[derive(Serialize)]
pub struct CreateRequest {
    emoji: Vec<u8>,
    name: String,
    aliases: Vec<String>
}

impl IntoRequest for CreateRequest {
    fn into_request(self, b: &impl AuthenticatedBuildRequestBuilder) -> Request {
	b.post("api/v1/emoji-custom.create").form(&self).build().unwrap()
    }
}

impl FromResponse for CreateRequest {
    type Output = WithSuccess<()>;

    fn from_response(response: Response) -> Option<Self::Output> {
	response.json().unwrap()
    }
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct DeleteRequest {
    emoji_id: String
}

impl IntoRequest for DeleteRequest {
    fn into_request(self, b: &impl AuthenticatedBuildRequestBuilder) -> Request {
	b.post("api/v1/emoji-custom.delete").json(&self).build().unwrap()
    }
}

impl FromResponse for DeleteRequest {
    type Output = WithSuccess<()>;

    fn from_response(response: Response) -> Option<Self::Output> {
	response.json().unwrap()
    }
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct UpdateRequest {
    #[serde(rename="_id")]
    id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    emoji: Option<Vec<u8>>,
    name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    aliases: Option<Vec<String>>
}

impl IntoRequest for UpdateRequest {
    fn into_request(self, b: &impl AuthenticatedBuildRequestBuilder) -> Request {
	b.post("api/v1/emoji-custom.update").form(&self).build().unwrap()
    }
}

impl FromResponse for UpdateRequest {
    type Output = WithSuccess<()>;

    fn from_response(response: Response) -> Option<Self::Output> {
	response.json().unwrap()
    }
}
