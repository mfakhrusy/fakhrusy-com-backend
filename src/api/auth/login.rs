use actix_web::error::BlockingError;
use actix_web::{HttpResponse, Result, web};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use serde::{Serialize, Deserialize};
use crate::model::user::{User};
use crate::schema::users::dsl::{email, users};

use crate::utils::{generate_jwt, verify_password};
use crate::{errors::ServiceError, model::db::Pool};

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

pub async fn login_handler(req: web::Json<LoginRequest>, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || query(req.into_inner(), pool)).await;

    match res {
        Ok(login_response) => return Ok(HttpResponse::Ok().json(login_response)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError)
        }

    }
}

fn query(data: LoginRequest, pool: web::Data<Pool>) -> Result<LoginResponse, ServiceError> {

    let conn: &PgConnection = &pool.get().unwrap();
    use crate::diesel::ExpressionMethods;

    let res = users.filter(email.eq(&data.email)).load::<User>(conn);

    match res {
        Err(diesel::result::Error::NotFound) => return Err(ServiceError::AuthenticationError),
        Err(_) => return Err(ServiceError::InternalServerError),
        Ok(mut current_users) => {
            if let Some(user) = current_users.pop() {
                match verify_password(&data.password, &user.hashed_password) {
                    Ok(_) => {
                        let jwt_token = generate_jwt(&data.email)?;
                        
                        return Ok(LoginResponse {
                            token: jwt_token,
                            email: user.email,
                            full_name: user.full_name.unwrap_or_default()
                        })
                    },
                    Err(_) => return Err(ServiceError::AuthenticationError)
                }
            }

            return Err(ServiceError::InternalServerError);
        }
    }

}