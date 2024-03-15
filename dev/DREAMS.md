# Dreams [WIP]

These are my hopes and dreams when it comes to an s3-compat proxy/cache/etc.

## Phase 1 - Simple Pull-Through Cache

### Goal

The goal of this phase is simple: 

**_Provide an easy way to minimize the downsides of querying data straight from s3 using embedded olap systems such as DataFusion, DuckDB, chDB, Velox, etc._**


### Features

* Configurable cache backends
    * in-memory
    * local filesystem
    * s3 express
    * dynamodb? something else?
* Lightweight aliasing
    * Make `s3://tblA/$YEAR/$MONTH/$DAY` an alias of `s3://somebucket/somepath/$YEAR/$MONTH/$DAY`
    * Make `s3://tblB/` an alias of `s3://somebucket/anotherpath/`
* Multiple (x-object store) backends
    * Alias `s3://tblC` to `gcs://somebucket/somepath`
    * Alias `s3://tblD` to `s3://yeesh/`
* Alias and object-level caching configuration
    * Cache all objects from `s3://pathA` for 30 minutes
    * LRU-cache `s3://pathB` to a maximum of 20 gb
    * Cache and reload all objects from `s3://pathC` every hour
* Auth
    * Passthrough to aws/gcs iam (for now)
* Object-level purge/warm endpoint(s)
    * Purge and optionally reload objects from cache when
* Alias-level purge/warm endpoint(s)
    * Ditto to `object-level` purging, but incorporate it for aliases.
* Lightweight object cataloging
    * Speed up `list` operations such as `aws s3 ls`
    * Incremental additions using object/bucket notifications

## Phase 2 - Fancy Read Stuff

### Goal
The goal of this phase is to start leveraging the benefits of having a sql engine (`datafusion`) sitting between an s3-compatible shim (`sprox`) and `s3`/`gcs`/`azure blob`/`minio`/ etc itself.

### Features

* Read
    * Column/row-level obfuscation
    * Lightweight read-time data transformation
    * Enrichment-on-read using external sources
    * X-backend query routing to leverage hybrid storage
* Cataloging
    * Optimized onboard catalog updates (subscribe to bucket event notifications, etc)
* IAM
    * Fine-grained authorization

## Phase 3 - Write-through cache

### Goal
The goal of this phase is to make s3-based data management easier.

### Features
* Compaction
* Configurable cataloging
    * Delta table?
    * Iceberg?
    * Something else?
* Table versioning
