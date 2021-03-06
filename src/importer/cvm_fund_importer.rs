use super::cvm_fund_import_file_parser::parse_import_files;
use super::cvm_fund_price_file_parser::parse_price_files;
use crate::{
    models::{Funds, NewCvmFundImporterLog, NewFundPrice},
    repositories::importer_logs::*,
    repositories::securities::*,
    DbPool,
};
use chrono::NaiveDateTime;
use futures::{future::join_all, StreamExt};
use serde::Deserialize;

static CVM_URL: &str = "http://dados.cvm.gov.br/dados/FI/DOC/INF_DIARIO/DADOS/";

#[derive(Debug)]
pub enum ImporterError {
    AsyncTokioError,
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

#[derive(Debug, Clone)]
pub struct ImportFile {
    pub name: String,
    pub time: NaiveDateTime,
}

pub async fn import_cvm_fund_prices(pool: DbPool) -> Result<(), Vec<ImporterError>> {
    let errors = match parse_import_files(CVM_URL).await {
        Ok(import_file_list) => {
            let tasks = filter_pending_imported_files(&pool, import_file_list)
                .await
                .into_iter()
                .map(|import_file| {
                    let task_pool = pool.clone();
                    tokio::spawn(async move { handle_file_import(task_pool, import_file).await })
                })
                .collect::<Vec<_>>();

            join_all(tasks)
                .await
                .into_iter()
                .filter_map(|task_result| match task_result {
                    Ok(import_result) => import_result.err(),
                    Err(_) => Some(ImporterError::AsyncTokioError),
                })
                .collect()
        }
        Err(importer_error) => vec![importer_error],
    };

    if errors.len() >= 1 {
        Err(errors)
    } else {
        Ok(())
    }
}

async fn handle_file_import(pool: DbPool, import_file: ImportFile) -> Result<(), ImporterError> {
    let file_name = import_file.name.clone();
    println!("started importing file {}", file_name);

    let parsed_prices = parse_price_files(CVM_URL, &import_file.name).await?;
    persist_fund_prices(&pool, parsed_prices).await?;
    persist_imported_file_log(&pool, import_file).await?;

    println!("finished importing file {}", file_name);

    Ok(())
}

async fn persist_fund_prices(
    pool: &DbPool,
    parsed_fund_prices: Vec<ParsedFundPrice>,
) -> Result<(), ImporterError> {
    for fund_price in parsed_fund_prices {
        let cnpj = fund_price.cnpj.clone();
        match find_or_insert_fund(pool, cnpj.clone()).await {
            Ok(fund) => persist_fund_price(pool, fund, fund_price).await?,
            Err(_) => return Err(ImporterError::SaveFundError(cnpj)),
        };
    }

    Ok(())
}

async fn persist_fund_price(
    conn: &DbPool,
    fund: Funds,
    fund_price: ParsedFundPrice,
) -> Result<usize, ImporterError> {
    let new_fund_price = NewFundPrice::new(fund.id, fund_price.date, fund_price.price);
    insert_fund_price(conn, new_fund_price)
        .await
        .map_err(|_| ImporterError::SaveFundPriceError(fund_price))
}

async fn filter_pending_imported_files<'a>(
    pool: &DbPool,
    import_files: Vec<ImportFile>,
) -> Vec<ImportFile> {
    futures::stream::iter(import_files)
        .filter(|import_file| exist_import_file(pool, import_file.clone()))
        .collect()
        .await
}

async fn exist_import_file<'a>(pool: &DbPool, import_file: ImportFile) -> bool {
    find_cvm_fund_importer_log_by_name_and_time(&pool, import_file.name, import_file.time)
        .await
        .is_err()
}

async fn persist_imported_file_log(
    pool: &DbPool,
    import_file: ImportFile,
) -> Result<usize, ImporterError> {
    let file_name = import_file.name.clone();
    insert_cvm_fund_importer_log(
        pool,
        NewCvmFundImporterLog {
            file_name: import_file.name,
            file_last_modified: import_file.time,
            imported_at: chrono::offset::Utc::now().naive_utc(),
        },
    )
    .await
    .map_err(|_| ImporterError::SaveLogError(file_name))
}
