use supabasic_rs::Supabase;
use dotenvy::dotenv;
use std::env;
use serde_json::json;

#[tokio::test]
async fn test_select_and_insert() {
    dotenv().ok();
    let url = env::var("SUPABASE_URL").unwrap();
    let key = env::var("SUPABASE_KEY").unwrap();

    let client = Supabase::new(&url, &key);

    // Insert a row into your actual table
    let new_row = json!({ "name": "test-user", "role": "tester" });
    let _ = client
        .from("user_entities")   // 👈 changed here
        .insert(new_row)
        .execute()
        .await
        .unwrap();

    // Select rows back
    let rows = client
        .from("user_entities")   // 👈 changed here
        .select("*")
        .execute()
        .await
        .unwrap();

    println!("Rows: {:#?}", rows);

    assert!(rows.is_array());
}
