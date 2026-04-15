use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use uuid::Uuid;
use crate::handlers::auth::Claims;


pub struct AuthUser {
    pub user_id: Uuid,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where 
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
       // GET THE AUTHORIZATION HEADER
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(||{
                (StatusCode::UNAUTHORIZED, Json(json!({"error": "Missing authorization header"}))).into_response()
            })?;
        // EXTRACT TOKEN FROM BEARER <TOKEN>    
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(||{
            (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid authorization format"}))).into_response()
            })?;
        // VERIFY AND DECODE TOKEN
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        ).map_err(|_|{
            (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid token"}))).into_response()
        })?;

        //return authuser with the user_id from claims
        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_|{
                (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid user id in token"}))).into_response()
            })?;

        Ok(AuthUser{ user_id})   
        
    }
}