#![allow(unused)]
pub mod lib;
use lib::*;

use rusoto_dynamodb::DynamoDb;
use serde::{Serialize, Deserialize};

// const AWS_REGION: rusoto_core::Region = rusoto_core::Region::UsWest2;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Account {
//     id: String,
//     note: String,
// }


// fn main() {
//     let db = rusoto_dynamodb::DynamoDbClient::new(AWS_REGION.clone());

//     let input = rusoto_dynamodb::PutItemInput {
//         item: fields!{
//             id => "test",
//             note => String::new(),
//         },
//         table_name: String::from("scratch"),
//         ..Default::default()
//     };
//     let result = db
//         .put_item(input)
//         .sync()
//         .expect("dynamodb put request failed");
    


//     let input = rusoto_dynamodb::GetItemInput {
//         key: fields!{
//             id => "test"
//         },
//         table_name: String::from("scratch"),
//         ..Default::default()
//     };
//     // GO!
//     let result = db
//         .get_item(input)
//         .sync()
//         .ok()
//         .and_then(|output| output.item)
//         .and_then(|output| dynamodb_data::from_fields::<Account>(output).ok());
//     println!("result: {:#?}", result);
// }

// fn get() {
//     let db = rusoto_dynamodb::DynamoDbClient::new(AWS_REGION.clone());

//     let input = rusoto_dynamodb::GetItemInput {
//         key: fields!{
//             id => "test"
//         },
//         table_name: String::from("scratch"),
//         ..Default::default()
//     };
//     // GO!
//     let result = db
//         .get_item(input)
//         .sync()
//         .ok()
//         .and_then(|output| output.item)
//         .and_then(|output| dynamodb_data::from_fields::<serde_json::Value>(output).ok());
//     println!("result: {:#?}", result);
// }

// fn put() {
//     let db = rusoto_dynamodb::DynamoDbClient::new(AWS_REGION.clone());

//     let input = rusoto_dynamodb::PutItemInput {
//         item: fields!{
//             id => "test",
//             note => String::new(),
//         },
//         table_name: String::from("scratch"),
//         ..Default::default()
//     };
//     let result = db
//         .put_item(input)
//         .sync()
//         .expect("dynamodb put request failed");
// }


fn main() {
    // put();
    // get();
    // let x = "\0";
    // println!("x: {}", x);
}