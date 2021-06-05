use diesel::{result::Error, RunQueryDsl};

use crate::{models::CvmFundImporterLogs, schema::cvm_fund_importer_logs::dsl::*};
use crate::{models::NewCvmFundImporterLog, DbConn};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};

pub fn insert_cvm_fund_importer_log(
    conn: &DbConn,
    fund_importer_log: &NewCvmFundImporterLog,
) -> Result<usize, Error> {
    diesel::insert_into(cvm_fund_importer_logs)
        .values(fund_importer_log)
        .execute(conn)
}

pub fn find_cvm_fund_importer_log_by_name_and_time(
    conn: &DbConn,
    name: &String,
    time: &chrono::NaiveDateTime,
) -> Result<CvmFundImporterLogs, Error> {
    cvm_fund_importer_logs
        .filter(file_name.eq(&name).and(file_last_modified.eq(&time)))
        .first::<CvmFundImporterLogs>(conn)
}
