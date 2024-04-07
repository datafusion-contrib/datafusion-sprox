# Get Sprox Running
The following command runs `sprox` locally, using the `data` directory as S3 root.

    $ RUST_LOG=debug cargo run --features=binary data --access-key=A --secret-key=B


# Throw AWS config into env so it doesn't need to be sourced continuously:
    $ export AWS_ACCESS_KEY_ID=A
    $ export AWS_SECRET_ACCESS_KEY=B
    $ export AWS_ENDPOINT=http://localhost:8080


# List buckets and objects in bucket:

**List buckets:**

```
aws s3 ls --endpoint-url=http://localhost:8080/

2024-04-07 10:27:29 sprox
```

**List objects in `sprox` bucket:**

```
aws s3 ls s3://sprox/ --endpoint-url=http://localhost:8080/

2024-04-07 10:13:13       5217 sample.parquet
```


# Query sample data in local sprox bucket using DuckDB:

```
duckdb

CREATE SECRET (
    TYPE S3,
    PROVIDER CREDENTIAL_CHAIN,
    ENDPOINT 'localhost:8080',
    USE_SSL false, 
    URL_STYLE path
);


select * from read_parquet('s3://sprox/sample.parquet');

┌──────────────────────┬──────────────────────┬──────────────────────┬───┬──────────────────────┬──────────────────────┬──────────────────────┐
│         uuid         │      timestamp       │     buzTimestamp     │ … │   validationError    │       contexts       │       payload        │
│         uuid         │      timestamp       │      timestamp       │   │ struct(errortype v…  │ struct("io.silvert…  │ struct("action" va…  │
├──────────────────────┼──────────────────────┼──────────────────────┼───┼──────────────────────┼──────────────────────┼──────────────────────┤
│ a3ff45f6-9c92-4a98…  │ 2023-04-18 01:35:2…  │ 2023-04-18 01:35:2…  │ … │ {'errorType': inva…  │ {'io.silverton/buz…  │ {'action': didSome…  │
│ 288c1c89-212a-4e97…  │ 2023-04-18 01:35:2…  │ 2023-04-18 01:35:2…  │ … │ {'errorType': inva…  │ {'io.silverton/buz…  │ {'action': didSome…  │
│ 6165a3c7-5207-4ae2…  │ 2023-04-18 01:35:2…  │ 2023-04-18 01:35:2…  │ … │ {'errorType': inva…  │ {'io.silverton/buz…  │ {'action': didSome…  │
│ 5693548e-3017-4a3e…  │ 2023-04-18 01:35:2…  │ 2023-04-18 01:35:2…  │ … │ {'errorType': inva…  │ {'io.silverton/buz…  │ {'action': didSome…  │
├──────────────────────┴──────────────────────┴──────────────────────┴───┴──────────────────────┴──────────────────────┴──────────────────────┤
│ 4 rows                                                                                                                 15 columns (6 shown) │
└─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

```

# Query sample data in local sprox bucket using DataFusion (💀):

```
datafusion-cli


CREATE EXTERNAL TABLE sample
STORED AS PARQUET
OPTIONS(
    'aws.access_key_id' 'A',
    'aws.secret_access_key' 'B',
    'aws.oss.endpoint' 'http://localhost:8080'
)
LOCATION 'oss://sprox/sample.parquet';
Object Store error: Generic S3 error: Error after 0 retries in 2.291µs, max_retries:10, retry_timeout:180s, source:builder error for url (http://localhost:8080/sample.parquet): URL scheme is not allowed
```
