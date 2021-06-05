use super::cvm_fund_importer::{ImporterError, ParsedFundPrice};

impl From<csv::Error> for ImporterError {
    fn from(_error: csv::Error) -> Self {
        ImporterError::FundPriceParseError
    }
}

pub async fn parse_price_files(
    url: &str,
    name: &str,
) -> Result<Vec<ParsedFundPrice>, ImporterError> {
    let body = reqwest::get(format!("{}{}", url, name))
        .await?
        .text()
        .await?;

    println!("downloaded file {}", name);

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(body.as_bytes());

    let mut fund_prices = vec![];
    for result in reader.deserialize() {
        let fund_price = result?;
        fund_prices.push(fund_price);
    }

    Ok(fund_prices)
}
