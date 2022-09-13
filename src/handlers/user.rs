use axum::Extension;
use axum::http::Response;
use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::{DEFAULT_COST, hash, verify};
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, EntityTrait};
use uuid::Uuid;

use crate::models::PostUser;
use entity::user;

pub async fn create_user(
    Json(payload): Json<PostUser>,
    Extension(ref connection): Extension<DatabaseConnection>
) -> (StatusCode, String) {
    println!("{:#?}", payload);

    let new_user = user::ActiveModel {
        user_id: Set(Uuid::new_v4().to_owned()),
        avatar: Set("SomethingIWillReplaceLater".to_owned()),
        email: Set(payload.email.to_owned()),
        username: Set(payload.username.to_owned()),
        password: Set(hash(payload.password, DEFAULT_COST).unwrap().to_owned()),
        ..Default::default()
    };
    
    match user::Entity::insert(new_user).exec(connection).await {
        Ok(_) => (StatusCode::CREATED, "Record was created.".to_string()),
        Err(err) => {
            println!("{:#?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Record was not created.".to_string())
        }
    }
}
