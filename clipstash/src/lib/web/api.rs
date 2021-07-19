//! API routing, errors, and data structures.

use crate::data::AppDatabase;
use crate::service;
use crate::service::action;
use crate::web::{HitCounter, PASSWORD_COOKIE};
use crate::ServiceError;
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use rocket::Responder;
use rocket::State;
use serde::Serialize;
use std::str::FromStr;

/// HTTP request header name to include an API key.
pub const API_KEY_HEADER: &str = "x-api-key";

/// The possible errors that can occur when accessing an `ApiKey`.
#[derive(Responder, Debug, thiserror::Error, Serialize)]
pub enum ApiKeyError {
    /// API key not found.
    #[error("API key not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(String),
    /// Invalid API key format.
    #[error("invalid API key format")]
    #[response(status = 400, content_type = "json")]
    DecodeError(String)
}

/// An API key that is used to access the API endpoints.
#[derive(Debug, Clone)]
pub struct ApiKey(Vec<u8>);

impl ApiKey {
    /// Convert raw bytes of the [`ApiKey`]into a [`String`]
    pub fn to_base64(&self) -> String {
        base64::encode(self.0.as_slice())
    }
    /// Extract the underlying [`Vector`](std::vec::Vec). 
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

/// The default implementation produces a new 128-bit [`ApiKey`].
impl Default for ApiKey {
    fn default() -> Self {
        let key = (0..16).map(|_| rand::random::<u8>()).collect();
        Self(key)
    }
}

impl FromStr for ApiKey {
    type Err = ApiKeyError;
    fn from_str(key: &str) -> Result<Self, Self::Err> {
        base64::decode(key)
            .map(ApiKey)
            .map_err(|e| Self::Err::DecodeError(e.to_string()))
    }
}

/// The possible errors that can occur when attempting to respond to a request.
#[derive(Responder, Debug, thiserror::Error)]
pub enum ApiError {
    /// Data not found.
    #[error("not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(Json<String>),

    /// Server error.
    #[error("server error")]
    #[response(status = 500, content_type = "json")]
    Server(Json<String>),

    /// Invalid submission by client.
    #[error("client error")]
    #[response(status = 401, content_type = "json")]
    User(Json<String>),

    /// Problem with the [`ApiKey`]
    #[error("key error")]
    #[response(status = 400, content_type = "json")]
    KeyError(Json<ApiKeyError>),
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::Clip(c) => Self::User(Json(format!("clip parsing error: {}", c))),
            ServiceError::NotFound => Self::NotFound(Json("entity not found".to_owned())),
            ServiceError::Data(_) => Self::Server(Json("a server error occurred".to_owned())),
            ServiceError::PermissionError(msg) => Self::User(Json(msg)),
        }
    }
}

/// Allows an [`ApiKey`] to be used as a [request guard](https://rocket.rs/v0.5-rc/guide/requests/#request-guards) in a route.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn server_error() -> Outcome<ApiKey, ApiError> {
            Outcome::Failure((
                Status::InternalServerError,
                ApiError::Server(Json("server error".to_string())),
            ))
        }
        fn key_error(e: ApiKeyError) -> Outcome<ApiKey, ApiError> {
            Outcome::Failure((Status::BadRequest, ApiError::KeyError(Json(e))))
        }
        match req.headers().get_one(API_KEY_HEADER) {
            None => key_error(ApiKeyError::NotFound("API key not found".to_string())),
            Some(key) => {
                let db = match req.guard::<&State<AppDatabase>>().await {
                    Outcome::Success(db) => db,
                    _ => return server_error(),
                };
                let api_key = match ApiKey::from_str(key) {
                    Ok(key) => key,
                    Err(e) => return key_error(e),
                };
                match action::api_key_is_valid(api_key.clone(), db.get_pool()).await {
                    Ok(valid) if valid => Outcome::Success(api_key),
                    Ok(valid) if !valid => {
                        key_error(ApiKeyError::NotFound("API key not found".to_owned()))
                    }
                    _ => server_error(),
                }
            }
        }
    }
}

/// Route to generate a new [`ApiKey`].
///
/// The key will be logged to the terminal for this demo application.
#[rocket::get("/key")]
pub async fn new_api_key(database: &State<AppDatabase>)
    -> Result<Json<&str>, ApiError>
{
    let api_key = action::generate_api_key(database.get_pool()).await?;
    println!("Api Key: {}", api_key.to_base64());
    Ok(Json("Api key generated. See logs for details."))
}

/// Route to retrieve an existing [`Clip`](crate::domain::Clip), based on it's [`ShortCode`](crate::ShortCode).
#[rocket::get("/<shortcode>")]
pub async fn get_clip(
    shortcode: &str,
    database: &State<AppDatabase>,
    cookies: &CookieJar<'_>,
    hit_counter: &State<HitCounter>,
    _api_key: ApiKey,
) -> Result<Json<crate::Clip>, ApiError> {
    use crate::domain::clip::field::Password;

    let req = service::ask::GetClip {
        shortcode: shortcode.into(),
        password: cookies
            .get(PASSWORD_COOKIE)
            .map(|cookie| cookie.value())
            .map(|raw_password| Password::new(raw_password.to_string()).ok())
            .flatten()
            .unwrap_or_else(Password::default)
    };
    let clip = action::get_clip(req, database.get_pool()).await?;
    hit_counter.hit(shortcode.into(), 1);
    Ok(Json(clip))
}

/// Route to add a new [`Clip`](crate::Clip).
#[rocket::post("/", data = "<req>")]
pub async fn new_clip(
    req: Json<service::ask::NewClip>,
    database: &State<AppDatabase>,
    _api_key: ApiKey,
) -> Result<Json<crate::Clip>, ApiError> {
    let clip = action::new_clip(req.into_inner(), database.get_pool()).await?;
    Ok(Json(clip))
}

/// Route to update an existing [`Clip`](crate::Clip).
#[rocket::put("/", data = "<req>")]
pub async fn update_clip(
    req: Json<service::ask::UpdateClip>,
    database: &State<AppDatabase>,
    _api_key: ApiKey,
) -> Result<Json<crate::Clip>, ApiError> {
    let clip = action::update_clip(req.into_inner(), database.get_pool()).await?;
    Ok(Json(clip))
}

/// The URI [`routes`](rocket::Route) which can be mounted by [`rocket`].
pub fn routes() -> Vec<rocket::Route> {
    rocket::routes!(get_clip, new_clip, update_clip, new_api_key)
}


pub mod catcher {
    //! Contains all the API catchers.
    use rocket::serde::json::Json;
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    /// Catch unhandled errors.
    #[catch(default)]
    fn default(req: &Request) -> Json<&'static str> {
        eprintln!("General error: {:?}", req);
        Json("something went wrong...")
    }

    /// Catch server errors.
    #[catch(500)]
    fn internal_error(req: &Request) -> Json<&'static str> {
        eprintln!("Internal error: {:?}", req);
        Json("internal server error")
    }

    /// Catch missing data errors.
    #[catch(404)]
    fn not_found() -> Json<&'static str> {
        Json("404")
    }

    /// Catch user request errors.
    #[catch(401)]
    fn request_error() -> Json<&'static str> {
        Json("request error")
    }

    /// Catch API key errors.
    #[catch(400)]
    fn missing_api_key() -> Json<&'static str> {
        Json("API key missing or invalid")
    }

    /// The [`catchers`](rocket::Catcher) which can be registered by [`rocket`].
    pub fn catchers() -> Vec<Catcher> {
        catchers![not_found, default, internal_error, missing_api_key, request_error]
    }
}