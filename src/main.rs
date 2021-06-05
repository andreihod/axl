use axl::importer;

#[tokio::main]
async fn main() {
    let pool = axl::initialize_pool();
    importer::import_cvm_fund_prices(&pool).await.unwrap();
}
