pub mod adm_visitors;
pub mod new_visitor;
mod auth;
pub mod login;


use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use askama::Template;
use axum::extract::{FromRef, FromRequestParts};
use axum::headers::{HeaderName};
use axum::http::HeaderValue;
use axum::http::request::Parts;
use axum::response::{IntoResponseParts, ResponseParts};
use bb8::{Pool};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<PostgresConnectionManager<NoTls>>,
    pub session_secret: String,
}


// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct DatabaseConnection(Pool<PostgresConnectionManager<NoTls>>);

#[axum::async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
    where
        AppState: FromRef<S>,
        S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        Ok(Self(app_state.pool))
    }
}


// Hypothetical helper type for setting a single header
struct SetHeader<'a>(&'a str, &'a str);

impl<'a> IntoResponseParts for SetHeader<'a> {
    type Error = (StatusCode, String);

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        match (self.0.parse::<HeaderName>(), self.1.parse::<HeaderValue>()) {
            (Ok(name), Ok(value)) => {
                res.headers_mut().insert(name, value);
            }
            (Err(_), _) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header name {}", self.0),
                ));
            }
            (_, Err(_)) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header value {}", self.1),
                ));
            }
        }

        Ok(res)
    }
}
