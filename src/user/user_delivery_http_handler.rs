use std::sync::Arc;

use actix_web::{get, post, web, Error, HttpResponse};

use crate::user::UserContainer;

use super::{user_domain::Model, UserUsecase};

#[derive(Clone)]
pub struct UserHttpHandler {
    user_usecase: Arc<dyn UserUsecase>,
}

impl UserHttpHandler {
    pub const fn new(user_usecase: Arc<dyn UserUsecase>) -> UserHttpHandler {
        UserHttpHandler { user_usecase }
    }

    pub async fn get_user(&self, uid: i32) -> Result<HttpResponse, Error> {
        match self.user_usecase.get_by_id(uid).await {
            Some(user) => Ok(HttpResponse::Ok().json(user)),
            None => Ok(HttpResponse::NotFound().json("User not found.")),
        }
    }

    pub async fn create_user(&self, form: web::Json<Model>) -> Result<HttpResponse, Error> {
        match self.user_usecase.create_user(form).await {
            Some(_) => Ok(HttpResponse::Ok().finish()),
            None => Ok(HttpResponse::NotAcceptable().json("Can't create user.")),
        }
    }
}

#[get("/users/{user_id}")]
async fn get_user(
    data: web::Data<UserContainer>,
    uid: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let delivery = &data.http_delivery;
    delivery.get_user(*uid).await
}

#[post("/users")]
async fn create_user(
    data: web::Data<UserContainer>,
    form: web::Json<Model>,
) -> Result<HttpResponse, Error> {
    let delivery = &data.http_delivery;
    delivery.create_user(form).await
}
