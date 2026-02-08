# UserRegoAPI

This is a user registration API programmed and built in Rust.

## Features
- Registers users using username and password
- Generates and returns JWT (JSON Web Token) for sessionless authentication
- Routes using Axum Router

## WARNING !
- Passwords are locally stored in plaintext during runtime, use at your own risk

## Project structure
```graphsql
src/
|-main.rs     # entrypoint
|-models.rs   # define data structures
|-handlers.rs # handles HTTP register fn
|-state.rs    # appstate
|-auth.rs     # authentication
```

## How to build (linux/bash)

### Prereqs
- rustc and cargo 1.93.0
- (optional) `curl` to test API

### Clone repo
```bash
git clone https://github.com/raph426/UserRegoAPI.git
cd user UserRegoAPI
```

### Create .env file
Create your own .env file in repo's root folder. This is needed to store the secret key that will be used in the code. 

Generate/create your own secret key and assign it to a variable called ' SECRET_KEY '

`SECRET_KEY = "<your secret key>"`

### Build and run

```bash
cargo build
cargo run
```

### Test

```bash
cargo test
```

## Example usage

### Registering a new user ( w/ curl )

```bash
curl -X POST http://127.0.0.1:3000/register -H "Content-Type: application/json" -d '{"username": "reinhard", "pwd": "kircheis488"}'
```

Expected response:

```sh
{"token": <jwt_token_here>}
```


