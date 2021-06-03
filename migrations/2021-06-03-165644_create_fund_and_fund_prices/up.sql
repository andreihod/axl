CREATE TABLE funds
(
  id   SERIAL PRIMARY KEY,
  cnpj text NOT NULL
);

CREATE INDEX IDX_funds_cnpj ON funds (cnpj);

CREATE TABLE fund_prices
(
  id      SERIAL PRIMARY KEY,
  fund_id INTEGER NOT NULL,
  date    DATE NOT NULL,
  price   DOUBLE PRECISION NOT NULL,
  CONSTRAINT fund_prices_date_fund_uniq UNIQUE ( date, fund_id ),
  CONSTRAINT FK_funds_fund_prices FOREIGN KEY ( fund_id ) REFERENCES funds ( id )
);

CREATE INDEX FK_IDX_funds_fund_prices ON fund_prices (fund_id);