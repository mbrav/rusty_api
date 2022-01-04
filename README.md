# rusty_api

A fast api, but a bit rusty

## Usage

### server

```
$ cargo run
# Started http server: 127.0.0.1:8080
```

### web client

-   [http://localhost:8080/async-body/bob](http://localhost:8080/async-body/bob)
-   [http://localhost:8080/user/bob/](http://localhost:8080/user/bob) text/plain download
-   [http://localhost:8080/test](http://localhost:8080/test) (return status switch GET or POST or other)
-   [http://localhost:8080/favicon](http://localhost:8080/favicon)
-   [http://localhost:8080/index](http://localhost:8080/static/index.html)
-   [http://localhost:8080/notexit](http://localhost:8080/static/404.html) display 404 page
-   [http://localhost:8080/error](http://localhost:8080/error) Panic after request
