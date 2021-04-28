use serde::Deserialize;

use reqwest::blocking::{Request, Response};
use super::{FromResponse, IntoRequest, WithSuccess, AuthenticatedBuildRequestBuilder};

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


#[derive(Debug)]
#[derive(Deserialize)]
pub struct EmojiResponseSets {
    update: Vec<EmojiData>,
    remove: Vec<EmojiData>
}

#[derive(Debug)]
#[derive(Deserialize)]
pub struct EmojiResponseData {
    emojis: EmojiResponseSets
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
