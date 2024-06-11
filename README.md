# icp-grader-be
Backend for grading papers online.

## Instructions to run
The program expects some environment variables. For local development,
I am doing it by having the following content on `.cargo/config.toml`

```toml
[env]
MONGO_CONN_URI = "mongodb://root:example@localhost:27017/"
DB_NAME = "icp_grader"
```

Then we start a database instance and eventually the program.

```bash
docker compose up --build
# then on a different terminal
cargo run
```
The service will be listening on `http://localhost:8080/`
