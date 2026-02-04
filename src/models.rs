use serde::{Deserialize, Serialize};

// JSON body client sends via POST
#[derive(Debug, Deserialize)]
pub struct RegiReq {
    pub username: String,
    pub pwd: String,
}

// JSON repsonse sent back to host
#[derive(Debug, Serialize)]
pub struct RegiResp {
    pub token: String,
}

// user is stored locally
#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub pwd: String, // !! shud not be in plaintext, HASH LATER
}
