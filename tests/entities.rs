// tests/entities.rs
use supabasic::Supabase;
use dotenvy::dotenv;
use std::env;

#[tokio::test]
async fn test_full_entity_lifecycle() {
    dotenv().ok();
    let url = env::var("SUPABASE_URL").unwrap();
    let key = env::var("SUPABASE_KEY").unwrap();
    let supa = Supabase::new(&url, &key);

    // 1. Create
    let entity_id = supa.create_entity("test-entity").await.unwrap();
    println!("Created entity_id = {}", entity_id);

    // 2. Fetch (active-only, should exist)
    let maybe_entity = supa.fetch_entity_by_id(entity_id).await.unwrap();
    println!("Fetched by id: {:?}", maybe_entity);
    assert!(maybe_entity.is_some());

    // 3. Soft delete
    let deleted = supa.soft_delete_entity(entity_id).await.unwrap();
    assert!(deleted, "Soft delete returned false");
    println!("Soft deleted entity {}", entity_id);

    // 4. Fetch raw (should still exist, but with deleted_at set)
    let maybe_raw = supa.fetch_entity_raw_by_id(entity_id).await.unwrap();
    println!("DEBUG raw after delete: {:?}", maybe_raw);
    assert!(
        maybe_raw.as_ref().map(|e| e.deleted_at.is_some()).unwrap_or(false),
        "deleted_at was not set by soft delete"
    );

    // 5. Fetch active (should now be filtered out)
    let maybe_entity_after = supa.fetch_entity_by_id(entity_id).await.unwrap();
    println!("DEBUG active after delete: {:?}", maybe_entity_after);
    assert!(
        maybe_entity_after.is_none(),
        "Entity was not excluded after delete (deleted_at filter failed)"
    );
}
