#![allow(dead_code)]

use std::env;

pub struct TestConfig {
    pub app_id: u32,
    pub item_id: u64,
    pub collection_id: u64,
}

impl TestConfig {
    pub fn load() -> Self {
        dotenv::dotenv().ok();

        let app_id = env::var("TEST_APP_ID")
            .expect("TEST_APP_ID environment variable must be set. Please copy .env.example to .env and configure your test values.")
            .parse()
            .expect("TEST_APP_ID must be a valid u32");

        let item_id = env::var("TEST_ITEM_ID")
            .expect("TEST_ITEM_ID environment variable must be set. Please copy .env.example to .env and configure your test values.")
            .parse()
            .expect("TEST_ITEM_ID must be a valid u64");

        let collection_id = env::var("TEST_COLLECTION_ID")
            .expect("TEST_COLLECTION_ID environment variable must be set. Please copy .env.example to .env and configure your test values.")
            .parse()
            .expect("TEST_COLLECTION_ID must be a valid u64");

        TestConfig {
            app_id,
            item_id,
            collection_id,
        }
    }
}
