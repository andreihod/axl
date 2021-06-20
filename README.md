## Axl

Axl is a data aggregator for the Brazilian financial market. Built for quantitative analisys.

### Running locally

Make sure you have the latest Rust stable version installed before proceeding.

1. Configure your PostgreSQL database using the `.env` file, you can start from the sample:

```
cp .env.sample .env
```

2. Install the diesel-cli and create the database:

```
cargo install diesel_cli
diesel setup
diesel migrate
```

3. Start the server

```
cargo run --release
```

The server will automatically start to update the historical prices, it can take some hours in the first start because there's a lot to download.

### Using with Python

This example retrieves the historical prices of two investment funds and creates a pandas dataframe:

```python
import pandas as pd

cnpjs = ['29.177.013/0001-12', '33.270.025/0001-64']

df = pd.read_json('http://localhost:8000/api/fund/prices?cnpj='+'&cnpj='.join(cnpjs))
df = df.pivot(index='date', columns='cnpj', values='price')
```
