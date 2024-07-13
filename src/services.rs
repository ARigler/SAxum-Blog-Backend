use axum::{response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};

use crate::auth::CurrentUser;

#[derive(Serialize, Deserialize)]
struct UserResponse {
    email: String,
}

pub async fn hello(Extension(currentUser): Extension<CurrentUser>) -> impl IntoResponse {
    Json(UserResponse {
        email: currentUser.email,
    })
}
