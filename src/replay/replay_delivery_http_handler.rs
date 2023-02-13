use std::sync::Arc;

use actix_multipart::Multipart;
use actix_web::{post, web, Error, HttpResponse};
use futures_util::TryStreamExt;
use uuid::Uuid;

use crate::replay::ReplayContainer;

use super::ReplayUsecase;

#[derive(Clone)]
pub struct ReplayHttpHandler {
    replay_usecase: Arc<dyn ReplayUsecase>,
}

impl ReplayHttpHandler {
    pub const fn new(replay_usecase: Arc<dyn ReplayUsecase>) -> ReplayHttpHandler {
        ReplayHttpHandler { replay_usecase }
    }

    pub async fn upload_replay(&self, mut payload: Multipart) -> anyhow::Result<HttpResponse> {
        let mut rofl_jsons = Vec::new();

        while let Some(mut field) = payload.try_next().await? {
            let file_name = field
                .content_disposition()
                .get_filename()
                .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

            log::info!("{file_name} uploaded.");

            let mut data: Vec<u8> = Vec::new();
            while let Some(bytes) = field.try_next().await? {
                data.append(&mut bytes.to_vec());
            }

            rofl_jsons.push(self.replay_usecase.get_json_from_rofl(&data).await?);
        }

        Ok(HttpResponse::Ok().json(rofl_jsons))
    }
}

#[post("/upload-replay")]
async fn upload_replay(
    data: web::Data<ReplayContainer>,
    payload: Multipart,
) -> Result<HttpResponse, Error> {
    let delivery = &data.http_delivery;
    Ok(delivery
        .upload_replay(payload)
        .await
        .expect("Can't upload replay file"))
}
