#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;

pub mod importer;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod schema;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenv::dotenv;
use rocket::{
    fairing::{AdHoc, Fairing},
    http::Status,
    outcome::Outcome,
};
use std::{env, sync::Arc};

pub type DbConn = diesel::PgConnection;
pub type DbPool = Arc<Pool<ConnectionManager<DbConn>>>;

pub fn initialize_service_pool() -> DbPool {
    dotenv().ok();
    let manager = ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    );

    Arc::new(
        Pool::builder()
            .max_size(20)
            .build(manager)
            .expect("Error building the service database pool"),
    )
}

#[derive(Clone)]
pub struct FairingDbPool(DbPool);

impl FairingDbPool {
    pub fn new(pool: DbPool) -> Self {
        FairingDbPool(pool)
    }

    pub fn get_pool(self) -> DbPool {
        self.0
    }
}

pub async fn db_pool_fairing(pool: FairingDbPool) -> impl Fairing {
    AdHoc::on_ignite("DbPool", |rocket| async { rocket.manage(pool) })
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for FairingDbPool {
    type Error = ();

    #[inline]
    async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, ()> {
        match request.rocket().state::<FairingDbPool>() {
            Some(state) => Outcome::Success(state.to_owned()),
            None => Outcome::Failure((Status::InternalServerError, ())),
        }
    }
}
