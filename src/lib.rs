pub mod client;
pub mod error;
pub mod entities;

pub use client::Supabase;
pub use error::{SupabasicError, Result};
pub use entities::{Entity, create_entity, fetch_entities};
