fn main() {
    // Temporarily disable Specta type generation until API is figured out
    // TODO: Fix Specta type generation
    // let types = specta::export();
    // specta_typescript::Typescript::default()
    //     .export_to("../src/lib/generated/types.ts", &types)
    //     .expect("Failed to export TypeScript types");

    tauri_build::build()
}
