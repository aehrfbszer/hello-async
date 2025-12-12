use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: String,
}

pub async fn get_user() -> Json<Vec<User>> {
    let v = (1..=300)
        .map(|id| User {
            id,
            name: format!("User{}", id),
        })
        .collect();
    tracing::info!("get_user handler called");
    Json(v)
}
