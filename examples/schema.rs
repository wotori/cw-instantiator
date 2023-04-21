// examples/schema.rs

use schemars::schema_for;
use serde_json::json;

use instantiator::{
    msg::InitMsg,
    state::State,
};

fn main() {
    // Generate schema for InitMsg
    let schema = schema_for!(InitMsg);
    let schema_json = json!(&schema).to_string();
    println!("InitMsg schema:\n{}", schema_json);

    // Generate schema for State
    let schema = schema_for!(State);
    let schema_json = json!(&schema).to_string();
    println!("\nState schema:\n{}", schema_json);
}
