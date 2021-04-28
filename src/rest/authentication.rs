use reqwest::blocking::{Request, Response};
use serde::{Deserialize, Serialize};

use super::{BuildRequestBuilder, FromResponse, IntoAnonymousRequest, IntoRequest, WithSuccess};

#[derive(Serialize)]
#[serde(untagged)]
pub enum AuthenticationRequest {
    Credentials {
	user: String,
	password: String
    },
    Resume(String)
}

#[derive(Deserialize)]
pub struct AuthenticationResponse {
    status: String,
    pub(crate) data: AuthenticationResponseData
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct AuthenticationResponseData {
    pub(crate) auth_token: String,
    pub(crate) user_id: String,
}

impl IntoAnonymousRequest for AuthenticationRequest {
    fn into_anonymous_request(self, b: &impl BuildRequestBuilder) -> Request {
	b.post_anonymous("/api/v1/login")
	    .json(&self)
	    .build()
	    .unwrap()
    }
}

impl FromResponse for AuthenticationRequest {
    type Output = AuthenticationResponse;

    fn from_response(response: Response) -> Option<Self::Output> {
	response.json().unwrap()
    }
}

