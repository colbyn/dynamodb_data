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

# WARNING:
**As we all know and love, DynamoDB rejects empty strings in favor of null without any metadata as to what the original type is… As a workaround I’m experimenting with encoding emptying strings VIA the ASCII null character (unless anyone has a better idea).**

This should presumably be more robust than using e.g. the [suggested idea here](https://stackoverflow.com/a/31174149). I haven’t read about this anywhere so perhaps I’m the first to use such a workaround. If the null encoding causes issues I can add a feature to disable the default behavior. 

