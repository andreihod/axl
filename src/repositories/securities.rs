use diesel::result::Error;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    models::{Funds, NewFund, NewFundPrice},
    schema::*,
    DbConn,
};

pub fn find_or_insert_fund(conn: &DbConn, cnpj: &String) -> Result<Funds, Error> {
    match find_fund_by_cnpj(conn, cnpj) {
        Ok(fund) => Ok(fund),
        Err(_) => {
            insert_fund(conn, cnpj)?;
            Ok(find_fund_by_cnpj(conn, cnpj)?)
        }
    }
}

pub fn find_fund_by_cnpj(conn: &DbConn, cnpj: &String) -> Result<Funds, Error> {
    funds::dsl::funds
        .filter(funds::dsl::cnpj.eq(cnpj))
        .first::<Funds>(conn)
}

pub fn insert_fund(conn: &DbConn, cnpj: &String) -> Result<usize, Error> {
    let new_fund = NewFund { cnpj };
    diesel::insert_into(funds::dsl::funds)
        .values(new_fund)
        .execute(conn)
}

pub fn insert_fund_price(conn: &DbConn, fund_price: &NewFundPrice) -> Result<usize, Error> {
    diesel::insert_into(fund_prices::dsl::fund_prices)
        .values(fund_price)
        .on_conflict((fund_prices::dsl::date, fund_prices::dsl::fund_id))
        .do_update()
        .set(fund_prices::dsl::price.eq(fund_price.price))
        .execute(conn)
}
