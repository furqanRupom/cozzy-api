use crate::models::user::{LoginUser, RegisterUser, UserLoginResponse, UserResponse};
use sqlx::{Pool, Postgres};

pub async fn register_user(
    pool: Pool<Postgres>,
    data: RegisterUser,
) -> Result<String, Box<dyn std::error::Error>> {
    let hashed = crate::utils::hash::hash_password(&data.password);

    let user_result = sqlx::query_as::<_, UserResponse>(
        "INSERT INTO users (email,password) VALUES ($1,$2) RETURNING id,email",
    )
    .bind(&data.email)
    .bind(hashed)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.code().as_deref() == Some("23505") {
                return "Email already exists".into();
            }
        }
        Box::new(e) as Box<dyn std::error::Error>
    });
    let user = user_result.unwrap();
    Ok(format!("User {} created successfully", user.email))
}

pub async fn login_user(
    pool: Pool<Postgres>,
    data: LoginUser,
) -> Result<String, Box<dyn std::error::Error>> {
    let user_result = sqlx::query_as::<_, UserLoginResponse>(
        "SELECT id,email,password FROM users WHERE email = $1",
    )
    .bind(&data.email)
    .fetch_one(&pool)
    .await;

    let user = user_result.unwrap();

    let verify = crate::utils::hash::verify_password(&data.password, &user.password);

    if verify {
        Ok(crate::utils::jwt::create_token(user.id.to_string()))
    } else {
        Ok("Invalid credentials".to_string())
    }
}
