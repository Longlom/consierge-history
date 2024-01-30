use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};

use serde::{Deserialize, Serialize};

use crate::db::Database;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct MsgData {
    pub token: String,
    pub user_message: String,
    pub consierge_message: String,
}


pub async fn save_message(
    State(db): State<Database>,
    Json(payload): Json<MsgData>,
) -> Result<impl IntoResponse, StatusCode> {

    match sqlx::query(
        "INSERT INTO chat_history (token, user_msg, consierge_msg)
    VALUES ($1, $2, $3)",
    )
    .bind(&payload.token)
    .bind(&payload.user_message)
    .bind(&payload.consierge_message)
    .execute(&db.connection_pool)
    .await
    {
        Ok(result) => {
            tracing::debug!("Successfully saved message - {}", result.rows_affected());
        }
        Err(err) => {
            tracing::error!("Error happened while saving message - {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };


    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
pub struct MessageParams {
    pub token: String,
}

#[derive(sqlx::FromRow, Clone, Deserialize, Serialize)]
struct DbMessageData {
    user_msg: String,
    consierge_msg: String,
}



pub async fn get_message(
    State(db): State<Database>,
    Query(params): Query<MessageParams>
) -> Result<impl IntoResponse, StatusCode> {


    let messages = match sqlx::query_as::<_, DbMessageData>(
        "SELECT user_msg, consierge_msg FROM chat_history WHERE (token = $1) ORDER BY timestamp",
    )
    .bind(&params.token)
    .fetch_all(&db.connection_pool)
    .await
    {
        Ok(result) => {
            if result.len() > 1 {
                tracing::debug!("Found {} messages", result.len());
            }

        result.clone()

        }
        Err(err) => {
            tracing::error!("Error happened while finding user - {}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };



    Ok((StatusCode::OK, axum::Json(messages).into_response()) )
}
