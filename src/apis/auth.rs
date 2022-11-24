use axum::extract::{FromRef, FromRequestParts, TypedHeader};
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use axum::http::request::Parts;
use axum::http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::apis::AppState;

#[derive(Clone, Debug)]
pub struct Session {
    pub uid: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    nbf: usize,
    // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct AuthSession(pub Session);

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthSession
    where
        AppState: FromRef<S>,
        S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // You can either call them directly...
        match TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await {
            Ok(TypedHeader(Authorization(token_encoded))) => {
                let app_state = AppState::from_ref(state);
                match decode::<Claims>(&token_encoded.token(),
                                       &DecodingKey::from_secret(&app_state.session_secret.as_bytes()),
                                       &Validation::default()) {
                    Ok(token) => {
                        Ok(Self(Session {
                            uid: token.claims.sub,
                            display_name: "from token".to_string(),
                        }))
                    }
                    Err(err) => Ok(Self(Session {
                        uid: "anonymous".to_string(),
                        display_name: err.to_string(),
                    })),
                }
            }
            Err(_) => {
                Ok(Self(Session {
                    uid: "anonymous".to_string(),
                    display_name: "没有Token".to_string(),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn token() {
        // HS256
        let token = encode(&Header::default(), &Claims {
            exp: time::OffsetDateTime::now_utc().unix_timestamp() as usize + 3600,
            nbf: 1,
            sub: "this is sub".to_string(),
        }, &EncodingKey::from_secret("secret".as_ref())).unwrap();
        // assert_eq!("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjAsIm5iZiI6MCwic3ViIjoidGhpcyBpcyBzdWIifQ.UU9d5Uxp28yG-OzklVAz42y28IKjpSy9ElwZRy-cwZk".to_string(), token);

        let token = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()).unwrap();
        assert_eq!("this is sub".to_string(), token.claims.sub)
    }
}
