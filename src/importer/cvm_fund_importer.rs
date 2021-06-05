use super::cvm_fund_import_file_parser::parse_import_files;
use super::cvm_fund_price_file_parser::parse_price_files;
use crate::{
    models::{Funds, NewCvmFundImporterLog, NewFundPrice},
    repositories::importer_logs::*,
    repositories::securities::*,
    DbConn,
};
use chrono::NaiveDateTime;
use serde::Deserialize;

static CVM_URL: &str = "http://dados.cvm.gov.br/dados/FI/DOC/INF_DIARIO/DADOS/";

#[derive(Debug)]
pub enum ImporterError {
    HttpError,
    ParseError,
    FundPriceParseError,
    SaveLogError(String),
    SaveFundError(String),
    SaveFundPriceError(ParsedFundPrice),
}

#[derive(Deserialize, Debug)]
pub struct ParsedFundPrice {
    #[serde(rename = "CNPJ_FUNDO")]
    cnpj: String,
    #[serde(rename = "DT_COMPTC")]
    date: chrono::NaiveDate,
    #[serde(rename = "VL_QUOTA")]
    price: f64,
}

#[derive(Debug)]
pub struct ImportFile {
    pub name: String,
    pub time: NaiveDateTime,
}

pub async fn import_cvm_fund_prices(conn: &DbConn) -> Result<(), ImporterError> {
    for import_file in filter_pending_imported_files(parse_import_files(CVM_URL).await?, &conn) {
        println!("importing file {}", import_file.name);
        persist_fund_prices(conn, parse_price_files(CVM_URL, &import_file.name).await?)?;
        persist_imported_file_log(conn, import_file)?;
    }

    Ok(())
}

fn persist_fund_prices(
    conn: &DbConn,
    parsed_fund_prices: Vec<ParsedFundPrice>,
) -> Result<(), ImporterError> {
    for fund_price in parsed_fund_prices {
        match find_or_insert_fund(conn, &fund_price.cnpj) {
            Ok(fund) => persist_fund_price(conn, fund, fund_price)?,
            Err(_) => return Err(ImporterError::SaveFundError(fund_price.cnpj)),
        };
    }

    Ok(())
}

fn persist_fund_price(
    conn: &DbConn,
    fund: Funds,
    fund_price: ParsedFundPrice,
) -> Result<usize, ImporterError> {
    let new_fund_price = &NewFundPrice::new(&fund.id, &fund_price.date, &fund_price.price);
    insert_fund_price(conn, new_fund_price)
        .map_err(|_| ImporterError::SaveFundPriceError(fund_price))
}

fn filter_pending_imported_files(import_files: Vec<ImportFile>, conn: &DbConn) -> Vec<ImportFile> {
    import_files
        .into_iter()
        .filter(|import_file| {
            find_cvm_fund_importer_log_by_name_and_time(conn, &import_file.name, &import_file.time)
                .is_err()
        })
        .collect()
}

fn persist_imported_file_log(
    conn: &DbConn,
    import_file: ImportFile,
) -> Result<usize, ImporterError> {
    insert_cvm_fund_importer_log(
        conn,
        &NewCvmFundImporterLog {
            file_name: &import_file.name,
            file_last_modified: &import_file.time,
            imported_at: &chrono::offset::Utc::now().naive_utc(),
        },
    )
    .map_err(|_| ImporterError::SaveLogError(import_file.name))
}
