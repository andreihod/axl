#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod importer;
pub mod models;
pub mod repositories;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

type DbConn = diesel::pg::PgConnection;

pub fn establish_connection() -> DbConn {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    DbConn::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
