use super::cvm_fund_import_file_parser::parse_import_files;
use super::cvm_fund_price_file_parser::parse_price_files;
use chrono::NaiveDateTime;
use serde::Deserialize;

static CVM_URL: &str = "http://dados.cvm.gov.br/dados/FI/DOC/INF_DIARIO/DADOS/";

#[derive(Debug)]
pub enum ImporterError {
    HttpError,
    ParseError,
    FundPriceParseError,
}

#[derive(Deserialize, Debug)]
pub struct FundPrice {
    #[serde(rename = "CNPJ_FUNDO")]
    cnpj: String,
    #[serde(rename = "VL_QUOTA")]
    price: f64,
}

#[derive(Debug)]
pub struct ImportFile {
    pub name: String,
    pub time: NaiveDateTime,
}

pub async fn update_fund_prices() -> Result<(), ImporterError> {
    let import_files = parse_import_files(CVM_URL).await?;

    // check what need to be imported

    let mut files_and_prices = vec![];
    for import_file in import_files {
        let fund_prices = parse_price_files(CVM_URL, &import_file.name).await?;
        println!("{:?} - {:?}", import_file, fund_prices);
        files_and_prices.push((import_file, fund_prices));
    }

    Ok(())
}
