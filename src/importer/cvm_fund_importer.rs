use super::cvm_fund_import_file_parser::parse_import_files;
use super::cvm_fund_price_file_parser::parse_price_files;
use crate::{
    models::{CvmFundImporterLogs, Funds, NewCvmFundImporterLog, NewFund, NewFundPrice},
    schema::cvm_fund_importer_logs::dsl::*,
    schema::fund_prices::dsl::*,
    schema::funds::dsl::*,
};
use chrono::NaiveDateTime;
use diesel::{BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
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

pub async fn update_fund_prices(conn: &PgConnection) -> Result<(), ImporterError> {
    let import_files = parse_import_files(CVM_URL).await?;

    for import_file in filter_pending_imported_files(import_files, &conn) {
        let parsed_fund_prices = parse_price_files(CVM_URL, &import_file.name).await?;
        println!("importing file {}", import_file.name);
        save_fund_prices(parsed_fund_prices, conn)?;
        save_imported_file_log(import_file, conn)?;
    }

    Ok(())
}

fn save_fund_prices(
    parsed_fund_prices: Vec<ParsedFundPrice>,
    conn: &PgConnection,
) -> Result<(), ImporterError> {
    for fund_price in parsed_fund_prices {
        let fund = find_or_insert_fund(&fund_price.cnpj, conn)?;
        insert_fund_price(fund, fund_price, conn)?;
    }

    Ok(())
}

fn find_or_insert_fund(fund_cnpj: &String, conn: &PgConnection) -> Result<Funds, ImporterError> {
    match find_fund_by_cnpj(fund_cnpj, conn) {
        Ok(fund) => Ok(fund),
        Err(_) => {
            save_fund(fund_cnpj, conn)?;
            Ok(find_fund_by_cnpj(fund_cnpj, conn).unwrap())
        }
    }
}

fn insert_fund_price(
    fund: Funds,
    fund_price: ParsedFundPrice,
    conn: &PgConnection,
) -> Result<(), ImporterError> {
    match diesel::insert_into(fund_prices)
        .values(NewFundPrice {
            fund_id: &fund.id,
            date: &fund_price.date,
            price: &fund_price.price,
        })
        .on_conflict((date, fund_id))
        .do_update()
        .set(price.eq(fund_price.price))
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(_) => Err(ImporterError::SaveFundPriceError(fund_price)),
    }
}

fn find_fund_by_cnpj(
    fund_cnpj: &String,
    conn: &PgConnection,
) -> Result<Funds, diesel::result::Error> {
    funds.filter(cnpj.eq(fund_cnpj)).first::<Funds>(conn)
}

fn filter_pending_imported_files(
    import_files: Vec<ImportFile>,
    conn: &PgConnection,
) -> Vec<ImportFile> {
    import_files
        .into_iter()
        .filter(|import_file| {
            let predicate = file_name
                .eq(&import_file.name)
                .and(file_last_modified.eq(&import_file.time));

            cvm_fund_importer_logs
                .filter(predicate)
                .first::<CvmFundImporterLogs>(conn)
                .is_err()
        })
        .collect()
}

fn save_fund(fund_cnpj: &String, conn: &PgConnection) -> Result<(), ImporterError> {
    match diesel::insert_into(funds)
        .values(NewFund { cnpj: fund_cnpj })
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(_) => Err(ImporterError::SaveFundError(String::from(fund_cnpj))),
    }
}

fn save_imported_file_log(
    import_file: ImportFile,
    conn: &PgConnection,
) -> Result<(), ImporterError> {
    match diesel::insert_into(cvm_fund_importer_logs)
        .values(NewCvmFundImporterLog {
            file_name: &import_file.name,
            file_last_modified: &import_file.time,
            imported_at: &chrono::offset::Utc::now().naive_utc(),
        })
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(_) => Err(ImporterError::SaveLogError(import_file.name)),
    }
}
