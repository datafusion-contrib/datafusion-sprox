# Get Sprox Running
The following command runs `sprox` locally, using the `data` directory as S3 root.

    $ RUST_LOG=debug cargo run  data --access-key=A --secret-key=B


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

# Query sample data in local sprox bucket using DataFusion:

Using Environment Variables:
```shell
AWS_ALLOW_HTTP=true  AWS_ACCESS_KEY_ID=A AWS_SECRET_ACCESS_KEY=B AWS_ENDPOINT=http://localhost:8080  datafusion-cli

> CREATE EXTERNAL TABLE sample
STORED AS PARQUET
LOCATION 's3://sprox/sample.parquet';
0 row(s) fetched.
Elapsed 0.226 seconds.

> SELECT * FROM sample;
+----------------------------------+----------------------------+----------------------------+------------+---------------+-------------+-------------+---------------------------------------------------+--------------+------------------------------+---------+---------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------+
| uuid                             | timestamp                  | buzTimestamp               | buzVersion | buzName       | buzEnv      | protocol    | schema                                            | vendor       | namespace                    | version | isValid | validationError                                                                                                                                                                                                                                | contexts                                                                                                                                                            | payload                                                               |
+----------------------------------+----------------------------+----------------------------+------------+---------------+-------------+-------------+---------------------------------------------------+--------------+------------------------------+---------+---------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------+
| a3ff45f69c924a98b4aab43698b6da74 | 2023-04-18T01:35:20.885810 | 2023-04-18T01:35:20.885810 | x.x.dev    | buz-bootstrap | development | cloudevents | io.silverton/buz/example/gettingStarted/v1.0.json | io.silverton | buz.example.quickstart.click | 1.0     | false   | {errorType: invalid payload, errorResolution: publish a valid payload, payloadValidationErrors: [{field: /, description: additional properties are not allowed, errorType: /: {"action":"didSometh... additional properties are not allowed}]} | {io.silverton/buz/internal/contexts/httpHeaders/v1.0.json: {Accept: */*, Content-Length: 915, Content-Type: application/cloudevents+json, User-Agent: curl/7.86.0}} | {action: didSomething, name: jakthom, somethingElse: bad, userId: 10} |
| 288c1c89212a4e97ba06c0fd67a74469 | 2023-04-18T01:35:20.885841 | 2023-04-18T01:35:20.885841 | x.x.dev    | buz-bootstrap | development | cloudevents | io.silverton/buz/example/gettingStarted/v1.0.json | io.silverton | buz.example.quickstart.click | 1.0     | false   | {errorType: invalid payload, errorResolution: publish a valid payload, payloadValidationErrors: [{field: /, description: additional properties are not allowed, errorType: /: {"action":"didSometh... additional properties are not allowed}]} | {io.silverton/buz/internal/contexts/httpHeaders/v1.0.json: {Accept: */*, Content-Length: 915, Content-Type: application/cloudevents+json, User-Agent: curl/7.86.0}} | {action: didSomething, name: jakthom, somethingElse: bad, userId: 10} |
| 6165a3c752074ae28bf8b39a303d9c23 | 2023-04-18T01:35:20.885852 | 2023-04-18T01:35:20.885852 | x.x.dev    | buz-bootstrap | development | cloudevents | io.silverton/buz/example/gettingStarted/v1.0.json | io.silverton | buz.example.quickstart.click | 1.0     | false   | {errorType: invalid payload, errorResolution: publish a valid payload, payloadValidationErrors: [{field: /, description: additional properties are not allowed, errorType: /: {"action":"didSometh... additional properties are not allowed}]} | {io.silverton/buz/internal/contexts/httpHeaders/v1.0.json: {Accept: */*, Content-Length: 915, Content-Type: application/cloudevents+json, User-Agent: curl/7.86.0}} | {action: didSomething, name: jakthom, somethingElse: bad, userId: 10} |
| 5693548e30174a3eb3180c5eb15ada30 | 2023-04-18T01:35:20.885870 | 2023-04-18T01:35:20.885870 | x.x.dev    | buz-bootstrap | development | cloudevents | io.silverton/buz/example/gettingStarted/v1.0.json | io.silverton | buz.example.quickstart.click | 1.0     | false   | {errorType: invalid payload, errorResolution: publish a valid payload, payloadValidationErrors: [{field: /, description: additional properties are not allowed, errorType: /: {"action":"didSometh... additional properties are not allowed}]} | {io.silverton/buz/internal/contexts/httpHeaders/v1.0.json: {Accept: */*, Content-Length: 915, Content-Type: application/cloudevents+json, User-Agent: curl/7.86.0}} | {action: didSomething, name: jakthom, somethingElse: bad, userId: 10} |
+----------------------------------+----------------------------+----------------------------+------------+---------------+-------------+-------------+---------------------------------------------------+--------------+------------------------------+---------+---------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------+
4 row(s) fetched.
Elapsed 0.017 seconds.
```

Using config options:
(note needs a newer version than datafusion `37.0.0`, due to [this issue])

[this issue]: https://github.com/apache/arrow-datafusion/issues/10072

```shell
datafusion-cli
DataFusion CLI v37.0.0
> CREATE EXTERNAL TABLE sample
STORED AS PARQUET
OPTIONS(
    'aws.access_key_id' 'A',
    'aws.secret_access_key' 'B',
    'aws.endpoint' 'http://localhost:8080',
    'aws.allow_http' 'true',
)
LOCATION 's3://sprox/sample.parquet';
0 row(s) fetched.
Elapsed 0.147 seconds.

> -- Query
SELECT * FROM sample;
+----------------------------------+----------------------------+----------------------------+------------+---------------+-------------+-------------+---------------------------------------------------+--------------+------------------------------+---------+---------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------+
| uuid                             | timestamp                  | buzTimestamp               | buzVersion | buzName       | buzEnv      | protocol    | schema                                            | vendor       | namespace                    | version | isValid | validationError                                                                                                                                                                                                                                | contexts                                                                                                                                                            | payload                                                               |
+----------------------------------+----------------------------+----------------------------+------------+---------------+-------------+-------------+---------------------------------------------------+--------------+------------------------------+---------+---------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------+
| a3ff45f69c924a98b4aab43698b6da74 | 2023-04-18T01:35:20.885810 | 2023-04-18T01:35:20.885810 | x.x.dev    | buz-bootstrap | development | cloudevents | io.silverton/buz/example/gettingStarted/v1.0.json | io.silverton | buz.example.quickstart.click | 1.0     | false   | {errorType: invalid payload, errorResolution: publish a valid payload, payloadValidationErrors: [{field: /, description: additional properties are not allowed, errorType: /: {"action":"didSometh... additional properties are not allowed}]} | {io.silverton/buz/internal/contexts/httpHeaders/v1.0.json: {Accept: */*, Content-Length: 915, Content-Type: application/cloudevents+json, User-Agent: curl/7.86.0}} | {action: didSomething, name: jakthom, somethingElse: bad, userId: 10} |
| 288c1c89212a4e97ba06c0fd67a74469 | 2023-04-18T01:35:20.885841 | 2023-04-18T01:35:20.885841 | x.x.dev    | buz-bootstrap | development | cloudevents | io.silverton/buz/example/gettingStarted/v1.0.json | io.silverton | buz.example.quickstart.click | 1.0     | false   | {errorType: invalid payload, errorResolution: publish a valid payload, payloadValidationErrors: [{field: /, description: additional properties are not allowed, errorType: /: {"action":"didSometh... additional properties are not allowed}]} | {io.silverton/buz/internal/contexts/httpHeaders/v1.0.json: {Accept: */*, Content-Length: 915, Content-Type: application/cloudevents+json, User-Agent: curl/7.86.0}} | {action: didSomething, name: jakthom, somethingElse: bad, userId: 10} |
| 6165a3c752074ae28bf8b39a303d9c23 | 2023-04-18T01:35:20.885852 | 2023-04-18T01:35:20.885852 | x.x.dev    | buz-bootstrap | development | cloudevents | io.silverton/buz/example/gettingStarted/v1.0.json | io.silverton | buz.example.quickstart.click | 1.0     | false   | {errorType: invalid payload, errorResolution: publish a valid payload, payloadValidationErrors: [{field: /, description: additional properties are not allowed, errorType: /: {"action":"didSometh... additional properties are not allowed}]} | {io.silverton/buz/internal/contexts/httpHeaders/v1.0.json: {Accept: */*, Content-Length: 915, Content-Type: application/cloudevents+json, User-Agent: curl/7.86.0}} | {action: didSomething, name: jakthom, somethingElse: bad, userId: 10} |
| 5693548e30174a3eb3180c5eb15ada30 | 2023-04-18T01:35:20.885870 | 2023-04-18T01:35:20.885870 | x.x.dev    | buz-bootstrap | development | cloudevents | io.silverton/buz/example/gettingStarted/v1.0.json | io.silverton | buz.example.quickstart.click | 1.0     | false   | {errorType: invalid payload, errorResolution: publish a valid payload, payloadValidationErrors: [{field: /, description: additional properties are not allowed, errorType: /: {"action":"didSometh... additional properties are not allowed}]} | {io.silverton/buz/internal/contexts/httpHeaders/v1.0.json: {Accept: */*, Content-Length: 915, Content-Type: application/cloudevents+json, User-Agent: curl/7.86.0}} | {action: didSomething, name: jakthom, somethingElse: bad, userId: 10} |
+----------------------------------+----------------------------+----------------------------+------------+---------------+-------------+-------------+---------------------------------------------------+--------------+------------------------------+---------+---------+------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------+---------------------------------------------------------------------------------------------------------------------------------------------------------------------+-----------------------------------------------------------------------+
4 row(s) fetched.
Elapsed 0.023 seconds.

```
