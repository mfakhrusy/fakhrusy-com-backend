use actix_web::FromRequest;
use futures::future::{ready, Ready};

use crate::model::{auth::AuthMiddlewareData, errors::GlobalServiceError};

pub struct AuthExtractor(Option<AuthMiddlewareData>);

impl FromRequest for AuthExtractor {
    type Error = GlobalServiceError;
    type Future = Ready<Result<Self, GlobalServiceError>>;
    type Config = ();

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let value = req.extensions().get::<AuthMiddlewareData>().cloned();

        return ready(Ok(AuthExtractor(value)));
    }
}

impl std::ops::Deref for AuthExtractor {
    type Target = Option<AuthMiddlewareData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
