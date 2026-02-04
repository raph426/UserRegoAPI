# UserRegoAPI

This is a user registration API programmed and built in Rust.

## features
- registers users with username and password
- generate JSONwebtokens for authentication
- uses Tokio crate for asynchronous capabilities
- Routes using Axum Router

## WARNING !
- passwords are currently stored in plaintext locally, use at your own risk
- secret key used to sign tokens is not secure and hard-coded

## project structure
```graphsql
src/
|-main.rs     # entrypoint
|-models.rs   # define data structures
|-handlers.rs # handles HTTP register fn
|-state.rs    # appstate
|-auth.rs     # authentication
```

## how to build (linux/bash)

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

## example usage

### registering a new user ( w/ curl )

```bash
curl -X POST http://127.0.0.1:3000/register -H "Content-Type: application/json" -d '{"username": "reinhard", "pwd": "kircheis488"}'
```

expected response:

```sh
{"token": <jwt_token_here>}
```


