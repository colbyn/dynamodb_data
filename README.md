Serde based serialization support and misc utilities for working with Dynamodb via the `rusoto_dynamodb` crate.

# Features:
* Automatically convert your types to/from `rusoto_dynamodb` data types VIA `serde_json`. This is an alternative to `serde_dynamodb` (which is mostly unimplemented).
* Query/Conversion Macros


# API Example:

```rust
use dynamodb_data::*;
use std::collections::HashMap;

let payload: HashMap<String, rusoto_dynamodb::AttributeValue> = fields!{
    id => ::uuid::Uuid::new_v4(),
    name => "user name",
    counter => 0
};
let get_item_query = rusoto_dynamodb::GetItemInput {
    key: fields!{
        id => ::uuid::Uuid::new_v4()
    },
    ..Default::default()
};
```