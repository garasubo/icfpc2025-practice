use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde_json::json;
use sqlx::MySqlPool;

use crate::models::{ApiResponse, CreateUserRequest, UpdateUserRequest, User};

pub async fn get_users(State(pool): State<MySqlPool>) -> Result<Json<ApiResponse<Vec<User>>>, StatusCode> {
    match sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
    {
        Ok(users) => Ok(Json(ApiResponse {
            success: true,
            data: Some(users),
            message: Some("Users retrieved successfully".to_string()),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_user(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
    {
        Ok(user) => Ok(Json(ApiResponse {
            success: true,
            data: Some(user),
            message: Some("User retrieved successfully".to_string()),
        })),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_user(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    match sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES (?, ?) RETURNING id, name, email, created_at, updated_at"
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .fetch_one(&pool)
    .await
    {
        Ok(user) => Ok(Json(ApiResponse {
            success: true,
            data: Some(user),
            message: Some("User created successfully".to_string()),
        })),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Err(StatusCode::CONFLICT)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_user(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<User>>, StatusCode> {
    let mut query = "UPDATE users SET ".to_string();
    let mut updates = Vec::new();
    let mut params = Vec::new();

    if let Some(name) = &payload.name {
        updates.push("name = ?");
        params.push(name.as_str());
    }

    if let Some(email) = &payload.email {
        updates.push("email = ?");
        params.push(email.as_str());
    }

    if updates.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    query.push_str(&updates.join(", "));
    query.push_str(" WHERE id = ?");

    let mut sql_query = sqlx::query(&query);
    for param in params {
        sql_query = sql_query.bind(param);
    }
    sql_query = sql_query.bind(id);

    match sql_query.execute(&pool).await {
        Ok(result) if result.rows_affected() == 0 => Err(StatusCode::NOT_FOUND),
        Ok(_) => {
            match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(&pool)
                .await
            {
                Ok(user) => Ok(Json(ApiResponse {
                    success: true,
                    data: Some(user),
                    message: Some("User updated successfully".to_string()),
                })),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_user(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(result) if result.rows_affected() == 0 => Err(StatusCode::NOT_FOUND),
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: None,
            message: Some("User deleted successfully".to_string()),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}