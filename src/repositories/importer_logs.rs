use crate::models::NewCvmFundImporterLog;
use crate::{models::CvmFundImporterLogs, schema::cvm_fund_importer_logs::dsl::*, DbPool};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl};
use tokio_diesel::*;

pub async fn insert_cvm_fund_importer_log(
    conn: &DbPool,
    fund_importer_log: NewCvmFundImporterLog,
) -> Result<usize, AsyncError> {
    diesel::insert_into(cvm_fund_importer_logs)
        .values(fund_importer_log)
        .execute_async(conn)
        .await
}

pub async fn find_cvm_fund_importer_log_by_name_and_time(
    conn: &DbPool,
    name: String,
    time: chrono::NaiveDateTime,
) -> Result<CvmFundImporterLogs, AsyncError> {
    cvm_fund_importer_logs
        .filter(file_name.eq(name).and(file_last_modified.eq(time)))
        .first_async::<CvmFundImporterLogs>(conn)
        .await
}
