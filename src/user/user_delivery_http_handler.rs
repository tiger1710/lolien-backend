use std::sync::Arc;

use actix_web::{get, post, web, Error, HttpResponse};

use super::{UserContainer, UserUsecase};

use entity::user;

#[derive(Clone)]
pub struct UserHttpHandler {
    user_usecase: Arc<dyn UserUsecase>,
}

impl UserHttpHandler {
    pub const fn new(user_usecase: Arc<dyn UserUsecase>) -> UserHttpHandler {
        UserHttpHandler { user_usecase }
    }

    pub async fn get_user(&self, uid: i32) -> anyhow::Result<HttpResponse> {
        match self.user_usecase.get_by_id(uid).await? {
            Some(user) => Ok(HttpResponse::Ok().json(user)),
            None => Ok(HttpResponse::NotFound().json("User not found.")),
        }
    }

    pub async fn create_user(&self, form: web::Json<user::Model>) -> anyhow::Result<HttpResponse> {
        self.user_usecase.create_user(form).await?;
        Ok(HttpResponse::Ok().finish())
    }
}

#[get("/users/{user_id}")]
async fn get_user(
    data: web::Data<UserContainer>,
    uid: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let delivery = &data.http_delivery;
    Ok(delivery.get_user(*uid).await.expect("Can't get user."))
}

#[post("/users")]
async fn create_user(
    data: web::Data<UserContainer>,
    form: web::Json<user::Model>,
) -> Result<HttpResponse, Error> {
    let delivery = &data.http_delivery;
    Ok(delivery
        .create_user(form)
        .await
        .expect("Can't create user."))
}
