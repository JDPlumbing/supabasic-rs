// tests/soft_delete.rs
use supabasic::Supabase;
use dotenvy::dotenv;
use std::env;

#[tokio::test]
async fn test_soft_delete_entity_only() {
    dotenv().ok();
    let url = env::var("SUPABASE_URL").unwrap();
    let key = env::var("SUPABASE_KEY").unwrap();
    let supa = Supabase::new(&url, &key);

    // Use an entity UUID that actually exists in your DB
    let id = uuid::Uuid::parse_str("70aa9b9c-0fd6-43e8-b73e-9fa4e17d9baa").unwrap();

    // Call the soft delete
    let success = supa.soft_delete_entity(id).await.unwrap();
    println!("Soft delete success? {}", success);

    // Fetch raw entity (without deleted_at filter)
    let raw = supa.fetch_entity_raw_by_id(id).await.unwrap();
    println!("DEBUG raw after delete: {:?}", raw);

    // Assert that deleted_at is set now
    if let Some(ent) = raw {
        assert!(ent.deleted_at.is_some(), "deleted_at was not set");
    } else {
        panic!("Entity not found at all after soft delete");
    }
}

