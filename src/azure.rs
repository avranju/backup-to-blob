use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
use std::time::Duration;

use async_std::task;
use derive_more::Display;
use futures::future::TryFutureExt;
use serde::{Deserialize, Serialize};
use surf::{self, http::StatusCode};

const LOGIN_SCOPE: &'static str = "https://storage.azure.com/user_impersonation";

#[derive(Serialize)]
struct DeviceCodeRequest<'a> {
    client_id: &'a str,
    scope: &'a str,
}

impl<'a> DeviceCodeRequest<'a> {
    fn new(client_id: &'a str, scope: &'a str) -> Self {
        DeviceCodeRequest { client_id, scope }
    }
}

#[derive(Deserialize, Debug)]
pub struct DeviceCodeResponse {
    device_code: String,
    expires_in: u64,
    interval: u64,
    message: String,
    user_code: String,
    verification_uri: String,
}

#[derive(Serialize)]
pub struct AuthTokenRequest<'a> {
    grant_type: &'a str,
    client_id: &'a str,
    device_code: &'a str,
}

impl<'a> AuthTokenRequest<'a> {
    fn new(client_id: &'a str, device_code: &'a str) -> Self {
        AuthTokenRequest {
            grant_type: "urn:ietf:params:oauth:grant-type:device_code",
            client_id,
            device_code,
        }
    }
}

#[derive(Deserialize)]
struct AuthTokenResponse {
    access_token: String,
    expires_in: u64,
    ext_expires_in: u64,
    scope: String,
    token_type: String,
}

#[derive(PartialEq, Debug)]
enum AuthErrorKind {
    Pending,
    InvalidGrant,
    ExpiredToken,
    Other(String),
}

impl Default for AuthErrorKind {
    fn default() -> Self {
        AuthErrorKind::Other("".to_owned())
    }
}

impl FromStr for AuthErrorKind {
    type Err = &'static str;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        let res = match inp {
            "authorization_pending" => AuthErrorKind::Pending,
            "invalid_grant" => AuthErrorKind::InvalidGrant,
            "expired_token" => AuthErrorKind::ExpiredToken,
            err => AuthErrorKind::Other(err.to_owned()),
        };

        Ok(res)
    }
}

#[derive(Deserialize, Default, Display)]
#[display(fmt = "{:?} {}", error, error_description)]
struct AuthError {
    correlation_id: String,
    #[serde(with = "serde_with::rust::display_fromstr")]
    error: AuthErrorKind,
    error_codes: Vec<i32>,
    error_description: String,
    error_uri: String,
    timestamp: String,
    trace_id: String,
}

impl<E> From<E> for AuthError
where
    E: Error + Display,
{
    fn from(err: E) -> Self {
        AuthError {
            error: AuthErrorKind::Other(err.to_string()),
            ..Default::default()
        }
    }
}

pub struct Login {
    tenant_id: String,
    client_id: String,
}

impl Login {
    pub fn new(tenant_id: String, client_id: String) -> Self {
        Login {
            tenant_id,
            client_id,
        }
    }

    pub async fn get_device_code(&self) -> Result<DeviceCodeResponse, AuthError> {
        let res = surf::post(format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/devicecode",
            self.tenant_id
        ))
        .body_form(&DeviceCodeRequest::new(&self.client_id, LOGIN_SCOPE))?
        .map_err(From::from)
        .await?
        .body_json()
        .await?;

        Ok(res)
    }

    pub async fn poll_response(&self, device_code: &str) -> Result<AuthTokenResponse, AuthError> {
        loop {
            match self.get_token(device_code).await {
                Ok(resp) => break Ok(resp),
                Err(err) if err.error == AuthErrorKind::Pending => {
                    task::sleep(Duration::from_secs(1)).await;
                    continue;
                }
                Err(err) => break Err(err),
            }
        }
    }

    async fn get_token(&self, device_code: &str) -> Result<AuthTokenResponse, AuthError> {
        let mut res = surf::post(format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
            self.tenant_id
        ))
        .body_form(&AuthTokenRequest::new(&self.client_id, device_code))?
        .await?;

        match res.status() {
            StatusCode::OK => Ok(res.body_json().await?),
            StatusCode::BAD_REQUEST => Err(res.body_json().await?),
            _ => Err(AuthError::from(res.body_string().await?)),
        }
    }
}
