use reqwest::blocking::{Request, Response};
use serde::{Deserialize, Serialize};

use super::{AuthenticatedBuildRequestBuilder, BuildRequestBuilder, FromResponse, IntoAnonymousRequest, IntoRequest};

#[derive(Serialize)]
#[serde(untagged)]
pub enum LoginRequest {
    Credentials {
	user: String,
	password: String
    },
    Resume(String)
}

#[derive(Deserialize)]
pub struct LoginResponse {
    status: String,
    pub(crate) data: LoginResponseData
}

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
pub struct LoginResponseData {
    pub(crate) auth_token: String,
    pub(crate) user_id: String,
}

impl IntoAnonymousRequest for LoginRequest {
    fn into_anonymous_request(self, b: &impl BuildRequestBuilder) -> Request {
	b.post_anonymous("/api/v1/login")
	    .json(&self)
	    .build()
	    .unwrap()
    }
}

impl FromResponse for LoginRequest {
    type Output = LoginResponse;

    fn from_response(response: Response) -> Option<Self::Output> {
	response.json().unwrap()
    }
}

pub struct LogoutRequest;

impl IntoRequest for LogoutRequest {
    fn into_request(self, b: &impl AuthenticatedBuildRequestBuilder) -> Request {
	b.post("/api/v1/logout").build().unwrap()
    }
}

impl FromResponse for LogoutRequest {
    type Output = ();
    
    fn from_response(_response: Response) -> Option<Self::Output> {
	Some(())
    }
}
