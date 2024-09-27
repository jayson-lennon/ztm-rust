//! All web-related things: API, templates, routing, workers.

pub mod api;
pub mod ctx;
pub mod form;
pub mod hitcounter;
pub mod http;
pub mod renderer;

pub use hitcounter::HitCounter;

/// Cookie name for storing password, after it is entered by the user.
pub const PASSWORD_COOKIE: &str = "password";

/// The possible errors that can occur when responding to an HTTP request.
///
/// This is only used when responding to page requests. See [`ApiError`](api::ApiError) for error responses from the API.
#[derive(rocket::Responder)]
pub enum PageError {
    /// A serialization error.
    #[response(status = 500)]
    Serialization(String),
    /// Problem rendering the page.
    #[response(status = 500)]
    Render(String),
    /// Data not found.
    #[response(status = 404)]
    NotFound(String),
    /// Server error.
    #[response(status = 500)]
    Internal(String),
}

impl From<handlebars::RenderError> for PageError {
    fn from(err: handlebars::RenderError) -> Self {
        PageError::Render(format!("{}", err))
    }
}

impl From<serde_json::Error> for PageError {
    fn from(err: serde_json::Error) -> Self {
        PageError::Serialization(format!("{}", err))
    }
}

#[cfg(test)]
pub mod test {
    use crate::test::async_runtime;
    use crate::RocketConfig;
    use rocket::local::blocking::Client;
    use tokio::runtime::Handle;

    pub fn init_test_client() -> (tokio::runtime::Runtime, Client) {
        let rt = async_runtime();
        let config = crate::web::test::config(rt.handle());
        let client = client(config);
        (rt, client)
    }

    pub fn config(handle: &Handle) -> RocketConfig {
        use crate::web::{hitcounter::HitCounter, renderer::Renderer};
        let renderer = Renderer::new("templates/".into());
        let database = crate::data::test::new_db(handle);
        let maintenance = crate::domain::maintenance::Maintenance::spawn(
            database.get_pool().clone(),
            handle.clone(),
        );
        let hit_counter = HitCounter::new(database.get_pool().clone(), handle.clone());

        RocketConfig {
            renderer,
            database,
            hit_counter,
            maintenance,
        }
    }

    pub fn client(config: RocketConfig) -> Client {
        Client::tracked(crate::rocket(config)).expect("failed to build rocket instance")
    }
}

