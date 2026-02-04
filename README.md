# UserRegoAPI

This is a user registration API programmed and built in Rust.

## features
- registers users with username and password
- generate JSONwebtokens for authentication
- uses Tokio crate for asynchronous capabilities
- Routes using Axum Router

## project structure
```graphsql
src/
|-main.rs     # entrypoint
|-models.rs   # define data structures
|-handlers.rs #
|-state.rs
|-auth.rs
```

## how to build

### prerequisites
- rustc and cargo 1.93.0
- (optional) `curl` to test API

### clone repo
```bash
git clone https://github.com/raph426/UserRegoAPI.git
cd user UserRegoAPI
```

### build and run

```bash
cargo build
cargo run
```

### test

```bash
cargo test
```

## usage

### registering a new user ( w/ curl )

```bash
curl -X POST http://127.0.0.1:3000/register -H "Content-Type: application/json" -d '{"username": "reinhard", "pwd": "kircheis488"}'
```

expected response:

```sh
{"token": <jwt_token_here>}
```


