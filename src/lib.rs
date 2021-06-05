#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod importer;
pub mod models;
pub mod repositories;
pub mod schema;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenv::dotenv;
use std::env;

type DbConn = diesel::PgConnection;
type DbPool = Pool<ConnectionManager<DbConn>>;

pub fn establish_connection() -> DbConn {
    let database_url = fetch_database_url();
    DbConn::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn initialize_pool() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(fetch_database_url());

    Pool::builder()
        .build(manager)
        .expect("Error building the database pool")
}

fn fetch_database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
