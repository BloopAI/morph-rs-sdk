# Rust API client for Morph

REST API for managing MorphVM instances and snapshots.

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project. By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

## Installation

```
morph-rs-sdk = { git = "https://github.com/BloopAI/morph-rs-sdk.git" }
```

## Usage

```rs
// Setup Morph
let morph_api_key = std::env::var("MORPH_API_KEY")
    .map_err(|_| "MORPH_API_KEY environment variable must be set")?;

let morph_configuration = morph_rs_sdk::apis::configuration::Configuration {
    base_path: "https://cloud.morph.so/api".to_owned(),
    user_agent: Some("OpenAPI-Generator/0.1.0/rust".to_owned()),
    client: reqwest::Client::new(),
    basic_auth: None,
    oauth_access_token: None,
    bearer_access_token: Some(morph_api_key),
    api_key: None,
};

let morph_usage =
    morph_rs_sdk::apis::usage_api::get_user_usage_user_usage_get(&morph_configuration, None)
        .await;

if let Ok(usage) = morph_usage {
    info!("Morph usage: {}", serde_json::to_string(&usage).unwrap());
}
```
