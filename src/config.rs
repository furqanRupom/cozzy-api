use std::env;

pub fn jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap()
}
