//! Main application library

pub mod data;
pub mod domain;
pub mod service;
pub mod web;

pub use domain::clip::field::ShortCode;
pub use domain::clip::{Clip, ClipError};
pub use domain::time::Time;
pub use data::DataError;
pub use service::ServiceError;

use data::AppDatabase;
use rocket::fs::FileServer;
use rocket::{Build, Rocket};
use web::{renderer::Renderer};
use web::hitcounter::HitCounter;
use domain::maintenance::Maintenance;

/// Creates a new Rocket build that is configured for running ClipStash.
pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .manage::<AppDatabase>(config.database)
        .manage::<Renderer>(config.renderer)
        .manage::<HitCounter>(config.hit_counter)
        .manage::<Maintenance>(config.maintenance)
        .mount("/", web::http::routes())
        .mount("/api/clip", web::api::routes())
        .mount("/static", FileServer::from("static"))
        .register("/", web::http::catcher::catchers())
        .register("/api/clip", web::api::catcher::catchers())
}

/// Data needed to set up ClipStash with Rocket.
pub struct RocketConfig {
    pub renderer: Renderer<'static>,
    pub database: AppDatabase,
    pub hit_counter: HitCounter,
    pub maintenance: Maintenance,
}


#[cfg(test)]
pub mod test {
    pub fn async_runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime")
    }
}