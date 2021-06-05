use crate::schema::cvm_fund_importer_logs;
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
