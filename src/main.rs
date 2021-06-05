use axl::importer;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let pool = axl::initialize_pool();

    rt.block_on(async {
        importer::import_cvm_fund_prices(pool).await.unwrap();
    })
}
