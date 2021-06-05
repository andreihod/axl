use crate::schema::fund_prices;
use diesel::Insertable;

#[derive(Insertable)]
#[table_name = "fund_prices"]
pub struct NewFundPrice {
    pub fund_id: i32,
    pub date: chrono::NaiveDate,
    pub price: f64,
}

impl NewFundPrice {
    pub fn new(fund_id: i32, date: chrono::NaiveDate, price: f64) -> Self {
        Self {
            fund_id,
            date,
            price,
        }
    }
}
