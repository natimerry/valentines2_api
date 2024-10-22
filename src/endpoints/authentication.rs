use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use sha2::digest::Update;
use sha2::{Digest, Sha256, Sha512};
use sqlx::PgPool;
use std::io::Read;
use tracing::{debug, info};
use validator::Validate;

use crate::{endpoints::ApiError, models::user::AuthUserSignupRequest};

fn sha256_hash(string: &str) -> String {
    let mut hasher = Sha512::new();
    Update::update(&mut hasher, string.as_bytes());
    let result = hasher.clone().finalize();
    format!("{:x}", result)
}

#[post("/register")]
async fn create_user(
    user: Json<AuthUserSignupRequest>,
    db: Data<PgPool>,
) -> Result<HttpResponse, ApiError> {
    debug!("{user:?}");
    user.validate()?;

    let hash = sha256_hash(&*user.password);
    debug!(hash);
    info!(user.email, user.username, "Registering new user");
    let db = db.clone();
    sqlx::query!(
        "INSERT INTO users(username,email,hashed_pass) VALUES ($1,$2,$3)",
        user.username,
        user.email,
        hash
    )
    .execute(&**db)
    .await?;
    Ok(HttpResponse::Ok().into())
}
