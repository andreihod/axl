table! {
    cvm_fund_importer_logs (id) {
        id -> Int4,
        file_name -> Varchar,
        file_last_modified -> Timestamp,
        imported_at -> Timestamp,
    }
}

table! {
    fund_prices (id) {
        id -> Int4,
        fund_id -> Int4,
        date -> Date,
        price -> Float8,
    }
}

table! {
    funds (id) {
        id -> Int4,
        cnpj -> Text,
    }
}

joinable!(fund_prices -> funds (fund_id));

allow_tables_to_appear_in_same_query!(
    cvm_fund_importer_logs,
    fund_prices,
    funds,
);
