mod importer;

#[tokio::main]
async fn main() {
    importer::update_fund_prices().await.unwrap();
}
