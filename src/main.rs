use axl::importer;

#[tokio::main]
async fn main() {
    let conn = axl::establish_connection();
    importer::import_cvm_fund_prices(&conn).await.unwrap();
}
