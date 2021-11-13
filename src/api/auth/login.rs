use crate::constants::MESSAGE_LOGIN_SUCCESS;
use crate::model::errors::Error;
use crate::model::response::ResponseBody;
use crate::model::user::User;
use crate::schema::users::dsl::{email, users};
use actix_web::error::BlockingError;
use actix_web::{web, HttpResponse, Result};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::utils::{generate_jwt, verify_password};
use crate::{model::db::Pool, model::errors::ServiceError};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    email: String,
    full_name: String,
    token: String,
}

pub async fn login_handler(
    req: web::Json<LoginRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || query(req.into_inner(), pool)).await;

    match res {
        Ok(login_response) => return Ok(HttpResponse::Ok().json(login_response)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn query(
    req: LoginRequest,
    pool: web::Data<Pool>,
) -> Result<ResponseBody<LoginResponse>, ServiceError> {
    let conn: &PgConnection = &pool.get().unwrap();
    use crate::diesel::ExpressionMethods;

    let res = users.filter(email.eq(&req.email)).load::<User>(conn);

    match res {
        Err(diesel::result::Error::NotFound) => {
            return Err(ServiceError::Unauthorized(Error::EmailOrPasswordMismatch))
        }
        Err(_) => return Err(ServiceError::InternalServerError),
        Ok(mut current_users) => {
            if let Some(user) = current_users.pop() {
                match verify_password(&req.password, &user.hashed_password) {
                    Ok(_) => {
                        let jwt_token = generate_jwt(&req.email)?;

                        return Ok(ResponseBody::new(
                            MESSAGE_LOGIN_SUCCESS,
                            Some(LoginResponse {
                                token: jwt_token,
                                email: user.email,
                                full_name: user.full_name.unwrap_or_default(),
                            }),
                            None,
                        ));
                    }
                    Err(_) => {
                        return Err(ServiceError::Unauthorized(Error::EmailOrPasswordMismatch))
                    }
                }
            }

            return Err(ServiceError::InternalServerError);
        }
    }
}
