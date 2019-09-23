#![allow(unused)]

use dynamodb_data::*;
use rusoto_dynamodb::DynamoDb;
use serde::{Serialize, Deserialize};

const AWS_REGION: rusoto_core::Region = rusoto_core::Region::UsWest2;
const TABLE_NAME: &str = "scratch";
const PRIMARY_KEY: &str = "id";
const IS_PRIMARY_KEY_ID: bool = true;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    id: String,
    note: String,
    ts: String,
    counter: u32,
}


fn get() {
    let db = rusoto_dynamodb::DynamoDbClient::new(AWS_REGION.clone());
    let input = rusoto_dynamodb::GetItemInput {
        key: fields!{
            (PRIMARY_KEY) => "test"
        },
        table_name: String::from(TABLE_NAME),
        ..Default::default()
    };

    // GO!

    // Maybe get as account type (since the account has an `id` field):
    if IS_PRIMARY_KEY_ID {
        // Example serializing to the `Account` type:
        let account = db
            .get_item(input.clone())
            .sync()
            .ok()
            .and_then(|output| output.item)
            .and_then(|output| dynamodb_data::from_fields::<Account>(output).ok());
        println!("result [as account]: {:#?}", account);
    }
    
    // Get whatever it is as json:
    let result = db
        .get_item(input)
        .sync()
        .ok()
        .and_then(|output| output.item)
        .and_then(|output| dynamodb_data::from_fields::<serde_json::Value>(output).ok());
    println!("result [as json]: {:#?}", result);
}

fn put() {
    let db = rusoto_dynamodb::DynamoDbClient::new(AWS_REGION.clone());

    let input = rusoto_dynamodb::PutItemInput {
        item: fields!{
            (PRIMARY_KEY) => "test",
            ts => "today",
            counter => 0,
            // Hmm giving DynamoDB an empty string...
            note => String::new(),
        },
        table_name: String::from(TABLE_NAME),
        ..Default::default()
    };
    let result = db
        .put_item(input)
        .sync()
        .expect("dynamodb put request failed");
}


fn main() {
    put();
    get();
}