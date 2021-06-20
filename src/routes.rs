use crate::{schema::*, FairingDbPool};
use diesel::Queryable;
use diesel::{ExpressionMethods, QueryDsl};
use rocket::http::Status;
use rocket::response::content::Json;
use serde::Serialize;
use tokio_diesel::*;

#[derive(Serialize, Queryable)]
struct FundPriceData {
    date: chrono::NaiveDate,
    cnpj: String,
    price: f64,
}

#[rocket::get("/fund/prices?<cnpj>")]
pub async fn prices(fairing: FairingDbPool, cnpj: Vec<String>) -> Result<Json<String>, Status> {
    fund_prices::dsl::fund_prices
        .inner_join(funds::dsl::funds)
        .filter(funds::dsl::cnpj.eq_any(cnpj))
        .select((
            fund_prices::dsl::date,
            funds::dsl::cnpj,
            fund_prices::dsl::price,
        ))
        .load_async::<FundPriceData>(&fairing.get_pool())
        .await
        .map(|data: Vec<FundPriceData>| Json(serde_json::to_string(&data).unwrap()))
        .map_err(|_| Status::InternalServerError)
}
