CREATE TABLE cvm_fund_importer_logs (
  id SERIAL PRIMARY KEY,
  file_name VARCHAR NOT NULL,
  file_last_modified TIMESTAMP NOT NULL,
  imported_at TIMESTAMP NOT NULL
);