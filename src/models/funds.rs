use crate::schema::funds;
use diesel::{Insertable, Queryable};

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
