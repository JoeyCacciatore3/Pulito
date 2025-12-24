// Simple test to verify Specta type generation works
use serde::{Deserialize, Serialize};
use specta::{TypeCollection, Type};
use specta_typescript::Typescript;

// Include the actual modules
mod cache;
mod commands;
mod db;
mod packages;
mod scanner;
mod trash;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct TestStruct {
    pub id: String,
    pub value: i32,
}

fn main() {
    let mut collection = TypeCollection::default();
    collection.register::<commands::SystemStats>();
    collection.register::<commands::AppSettings>();
    collection.register::<scanner::ScanItem>();
    let types = collection;

    match Typescript::default()
        .bigint(specta_typescript::BigIntExportBehavior::Number)
        .export_to("../src/lib/generated/types.ts", &types) {
        Ok(_) => {
            // Post-process the generated types to use undefined instead of null
            if let Ok(content) = std::fs::read_to_string("../src/lib/generated/types.ts") {
                let processed_content = content.replace(" | null", " | undefined");
                if let Err(e) = std::fs::write("../src/lib/generated/types.ts", processed_content) {
                    eprintln!("Failed to post-process TypeScript types: {}", e);
                } else {
                    println!("TypeScript types exported and post-processed successfully!");
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to export test types: {}", e);
        }
    }
}
