# Sprox

A DataFusion-powered S3 Proxy.

# Why Sprox?
Projects like [DataFusion](https://arrow.apache.org/datafusion/), [Velox](https://github.com/facebookincubator/velox), and [DuckDB](https://duckdb.org/) scream `just use S3!`.

But as awesome as S3 is, continuously `GET`ing files from it for local/embedded analytical workflows is silly ([and maybe even problematic](https://www.mdpi.com/2076-3417/11/18/8540)).

# Inspiration(s)

* [nginx](https://docs.nginx.com/nginx/admin-guide/web-server/)
