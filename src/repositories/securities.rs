use diesel::{ExpressionMethods, QueryDsl};
use tokio_diesel::{AsyncError, AsyncRunQueryDsl};

use crate::{
    models::{Funds, NewFund, NewFundPrice},
    schema::*,
    DbPool,
};

pub async fn find_or_insert_fund(pool: &DbPool, cnpj: String) -> Result<Funds, AsyncError> {
    match find_fund_by_cnpj(pool, cnpj.clone()).await {
        Ok(fund) => Ok(fund),
        Err(_) => {
            insert_fund(pool, cnpj.clone()).await?;
            Ok(find_fund_by_cnpj(pool, cnpj).await?)
        }
    }
}

pub async fn find_fund_by_cnpj(pool: &DbPool, cnpj: String) -> Result<Funds, AsyncError> {
    funds::dsl::funds
        .filter(funds::dsl::cnpj.eq(cnpj))
        .first_async::<Funds>(pool)
        .await
}

pub async fn insert_fund(pool: &DbPool, cnpj: String) -> Result<usize, AsyncError> {
    diesel::insert_into(funds::dsl::funds)
        .values(NewFund { cnpj })
        .execute_async(pool)
        .await
}

pub async fn insert_fund_price(
    pool: &DbPool,
    fund_price: NewFundPrice,
) -> Result<usize, AsyncError> {
    let price = fund_price.price.clone();
    diesel::insert_into(fund_prices::dsl::fund_prices)
        .values(fund_price)
        .on_conflict((fund_prices::dsl::date, fund_prices::dsl::fund_id))
        .do_update()
        .set(fund_prices::dsl::price.eq(price))
        .execute_async(pool)
        .await
}
