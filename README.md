# Sprox

A DataFusion-powered Serverless S3 Proxy.

# What is Sprox?

Sprox is a lightweight, serverless S3 proxy that sits between your favorite OLAP
engine and your object store.


```text
┌─────────────────────────────────────────────┐                                 
│ ┌─────────────────┐        ┌──────────────┐ │               .───────────.     
│ │                 │ S3 API │              │ │           _.─'             `──. 
│ │   "native" S3   │        │    Sprox     │ │          ;                     :
│ │   Application   │───────▶│              │─┼─────────▶:                     ;
│ │                 │        │              │ │           ╲                   ╱ 
│ └─────────────────┘        └───────┬──────┘ │            `──.           _.─'  
└────────────────────────────────────┼────────┘                `─────────'      
                                     │                                          
 Local Process / Pod                 │                                          
                                     ▼                       Object Store       
                                 .───────.                   (e.g. S3)          
                                (         )                                     
                                │`───────'│                                     
                                │         │                                     
                                │.───────.│                                     
                                (         )                                     
                                 `───────'                                      
                                  Cache on                                      
                               Local Storage                                    
                                (e.g. NVMe)                                     
```

# Why Sprox?

With Sprox you simply develop your application against the S3 API and don't
worry about the performance or cost implications of querying S3 directly.

Projects like [DataFusion](https://arrow.apache.org/datafusion/),
[Velox](https://github.com/facebookincubator/velox), and
[DuckDB](https://duckdb.org/) scream `just use S3!` and they are right. However,
as awesome as S3 is, continuously `GET`ing files from it for local/embedded
performance](https://www.mdpi.com/2076-3417/11/18/8540)) and cost as you pay a tithe on each request.

1. **Better**: Drop in replacement for S3 that is trivial to deploy and test (a single process that runs alongside your app).  
2. **Faster**: S3 API access for cached objects is as fast and predictable as local storage
2. **Cheaper**: Reduced number of `GET` requests (and thus your AWS bill).

# Features
* 100% Serverless (no containers to orchestrate)
* 100% pure Rust (and thus no runtime dependencies and easy to embed)

# Quick Start
(coming soonx)

# Get Involved

Sprox is still an early stage project. We welcome you to take a
look around, give it a test drive, and let us know what you think.

Read more in [README.md](crates/sprox/README.md) and [dev/DREAMS.md](dev/DREAMS.md).

# Inspiration(s)

* [nginx](https://docs.nginx.com/nginx/admin-guide/web-server/)
