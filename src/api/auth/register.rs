use crate::model::user::{NewUser, User};
use crate::schema::users::dsl::{email, users};
use crate::utils::{hash_password, validate_email};
use crate::{errors::ServiceError, model::db::Pool};
use actix_web::error::BlockingError;
use actix_web::{web, HttpResponse, Result};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
    full_name: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    email: String,
    full_name: String,
}

pub async fn register_handler(
    req: web::Json<RegisterRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || query(req.into_inner(), pool)).await;

    match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => match err {
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
            BlockingError::Error(err) => Err(err),
        },
    }
}

fn query(data: RegisterRequest, pool: web::Data<Pool>) -> Result<RegisterResponse, ServiceError> {
    let conn: &PgConnection = &pool.get().unwrap();
    use crate::diesel::ExpressionMethods;

    if !validate_email(&data.email) {
        return Err(ServiceError::BadRequest("Wrong E-mail Format".to_string()));
    }

    users
        .filter(email.eq(&data.email))
        .load::<User>(conn)
        .map_err(|_db_error| ServiceError::BadRequest("Invalid Data".into()))
        .and_then(|mut _result| {
            let password_and_salt = hash_password(&data.password)?;

            let hashed_password = password_and_salt.hashed_password;
            let salt = password_and_salt.salt;
            let new_user = NewUser {
                email: &data.email,
                hashed_password: &hashed_password,
                salt: &salt,
                full_name: &data.full_name,
            };

            let inserted_user: Result<usize, diesel::result::Error> = diesel::insert_into(users)
                .values(&vec![new_user])
                .execute(conn);

            match inserted_user {
                Err(_) => Err(ServiceError::InternalServerError),
                Ok(_) => Ok(RegisterResponse {
                    email: data.email,
                    full_name: data.full_name,
                }),
            }
        })
}
