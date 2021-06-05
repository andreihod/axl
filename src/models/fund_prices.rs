use crate::schema::fund_prices;
use diesel::Insertable;

#[derive(Insertable)]
#[table_name = "fund_prices"]
pub struct NewFundPrice<'a> {
    pub fund_id: &'a i32,
    pub date: &'a chrono::NaiveDate,
    pub price: &'a f64,
}

impl<'a> NewFundPrice<'a> {
    pub fn new(fund_id: &'a i32, date: &'a chrono::NaiveDate, price: &'a f64) -> Self {
        Self {
            fund_id,
            date,
            price,
        }
    }
}
