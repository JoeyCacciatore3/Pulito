// Simple test to verify Specta type generation works
use serde::{Deserialize, Serialize};
use specta::{TypeCollection, Type};
use specta_typescript::Typescript;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct TestStruct {
    pub id: String,
    pub value: i32,
}

fn main() {
    let mut collection = TypeCollection::default();
    collection.register::<TestStruct>();
    let types = collection;

    match Typescript::default().export_to("../src/lib/generated/test_types.ts", &types) {
        Ok(_) => {
            println!("Test types exported successfully!");
        }
        Err(e) => {
            eprintln!("Failed to export test types: {}", e);
        }
    }
}
