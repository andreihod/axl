#[macro_use]
extern crate rocket;

use axl::{importer, routes, FairingDbPool};

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let pool = axl::initialize_service_pool();

    rt.block_on(async {
        spawn_cvm_importer(pool.clone());
        launch_rocket(pool).await;
    })
}

fn spawn_cvm_importer(pool: axl::DbPool) {
    tokio::task::spawn(async { importer::import_cvm_fund_prices(pool).await.unwrap() });
}

async fn launch_rocket(pool: axl::DbPool) {
    rocket::build()
        .attach(axl::db_pool_fairing(FairingDbPool::new(pool)).await)
        .mount("/api", routes![routes::prices])
        .launch()
        .await
        .unwrap();
}
