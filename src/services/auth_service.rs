use crate::{
    error::AppError,
    models::user::{LoginUser, RegisterUser, UserLoginResponse, UserResponse},
};
use axum::http::StatusCode;
use sqlx::{Pool, Postgres};

pub async fn register_user(pool: Pool<Postgres>, data: RegisterUser) -> Result<String, AppError> {
    let hashed = crate::utils::hash::hash_password(&data.password);

    let user_result = sqlx::query_as::<_, UserResponse>(
        "INSERT INTO users (email,password) VALUES ($1,$2) RETURNING id,email",
    )
    .bind(&data.email)
    .bind(hashed)
    .fetch_one(&pool)
    .await
    .map_err(AppError::from);
    let user = user_result.unwrap();
    Ok(format!("User {} created successfully", user.email))
}

pub async fn login_user(pool: Pool<Postgres>, data: LoginUser) -> Result<String, AppError> {
    let user_result = sqlx::query_as::<_, UserLoginResponse>(
        "SELECT id,email,password FROM users WHERE email = $1",
    )
    .bind(&data.email)
    .fetch_one(&pool)
    .await;

    let user = match user_result {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return Err(AppError::new(StatusCode::UNAUTHORIZED, "User Not Found"));
        }
        Err(e) => {
            return Err(AppError::from(e)); // Handles other DB errors
        }
    };

    let verify = crate::utils::hash::verify_password(&data.password, &user.password);

    if verify {
        Ok(crate::utils::jwt::create_token(user.id.to_string()))
    } else {
        // Ok("Invalid credentials".to_string())
        Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid Credentials",
        ))
    }
}
