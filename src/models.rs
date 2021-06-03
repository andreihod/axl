use super::schema::{cvm_fund_importer_logs, fund_prices, funds};
use diesel::{Insertable, Queryable};

#[derive(Queryable)]
pub struct CvmFundImporterLogs {
    pub id: i32,
    pub file_name: String,
    pub file_last_modified: chrono::NaiveDateTime,
    pub imported_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "cvm_fund_importer_logs"]
pub struct NewCvmFundImporterLog<'a> {
    pub file_name: &'a str,
    pub file_last_modified: &'a chrono::NaiveDateTime,
    pub imported_at: &'a chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct Funds {
    pub id: i32,
    pub cnpj: String,
}

#[derive(Insertable)]
#[table_name = "funds"]
pub struct NewFund<'a> {
    pub cnpj: &'a str,
}

#[derive(Insertable)]
#[table_name = "fund_prices"]
pub struct NewFundPrice<'a> {
    pub fund_id: &'a i32,
    pub date: &'a chrono::NaiveDate,
    pub price: &'a f64,
}
