// https://github.com/SakaDream/actix-web-rest-api-with-jwt/blob/master/src/middleware/auth_middleware.rs
// copied from ^ with some changes

use crate::{
    constants,
    model::{auth::AuthMiddlewareData, db::Pool, response::ResponseBody},
    utils::decode_jwt,
};
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::{HeaderName, HeaderValue, Method},
    web::Data,
    Error, HttpMessage, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = false;

        let headers = req.headers_mut();
        headers.append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );

        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            for ignore_route in constants::AUTH_ROUTES.iter() {
                // Bypass some account routes
                if req.path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    break;
                }
            }

            if !authenticate_pass {
                if let Some(_pool) = req.app_data::<Data<Pool>>() {
                    // Connecting to database
                    if let Some(auth_header) = req.headers().get(constants::AUTHORIZATION) {
                        // Parsing authorization header
                        if let Ok(auth_str) = auth_header.to_str() {
                            if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                                // Parsing token
                                let token = auth_str[6..auth_str.len()].trim();
                                if let Ok(token_data) = decode_jwt(token.to_string()) {
                                    let email = token_data.claims.email;
                                    req.extensions_mut()
                                        .insert::<AuthMiddlewareData>(AuthMiddlewareData { email });
                                    authenticate_pass = true;
                                } else {
                                    // Invalid token
                                }
                            }
                        }
                    }
                }
            }
        }

        if authenticate_pass {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(ResponseBody::<()>::new(
                            constants::MESSAGE_INVALID_TOKEN,
                            None,
                            None,
                        ))
                        .into_body(),
                ))
            })
        }
    }
}
