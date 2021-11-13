use crate::constants::MESSAGE_GET_PROFILE_SUCCESS;
use crate::extractor::auth::AuthExtractor;
use crate::model::user::User;
use crate::model::{db::Pool, errors::ServiceError, response::ResponseBody};
use crate::schema::users::dsl::{email, users};
use actix_web::error::BlockingError;
use actix_web::{web, HttpResponse, Result};
use diesel::result::Error;
use diesel::PgConnection;
use diesel::{QueryDsl, RunQueryDsl};
use serde::Serialize;

#[derive(Serialize)]
pub struct MyProfileResponse {
    email: String,
    full_name: String,
}

pub async fn my_profile_handler(
    pool: web::Data<Pool>,
    auth_data: AuthExtractor,
) -> Result<HttpResponse, ServiceError> {
    let user_email = auth_data.as_ref().map(|x| x.email.clone());
    let res = web::block(move || query(user_email.unwrap_or_default(), pool)).await;

    match res {
        Ok(my_profile_response) => return Ok(HttpResponse::Ok().json(my_profile_response)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

pub fn query(
    user_email: String,
    pool: web::Data<Pool>,
) -> Result<ResponseBody<MyProfileResponse>, ServiceError> {
    let conn: &PgConnection = &pool.get().unwrap();
    use crate::diesel::ExpressionMethods;

    let res: Result<User, Error> = users.filter(email.eq(&user_email)).first(conn); //.load::<User>(conn);
    match res {
        Ok(user) => {
            let response = MyProfileResponse {
                email: user.email,
                full_name: user.full_name.unwrap_or_default(),
            };
            Ok(ResponseBody::new(
                MESSAGE_GET_PROFILE_SUCCESS,
                Some(response),
                None,
            ))
        }
        Err(e) => {
            println!("{:?}", e);
            Err(ServiceError::InternalServerError)
        }
    }
}
