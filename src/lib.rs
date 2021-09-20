use argon2::{Argon2, password_hash::{rand_core::OsRng, PasswordHasher, SaltString}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    #[serde(default = "Vec::new")]
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    password: String,
    pub permissions: Vec<String>,
}

impl User {
    pub fn new(email: String, password: String, permissions: Vec<String>) -> User {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        // TODO: Handle errors on hash_password
        // TODO: Verify hash

        User {
            id: Uuid::new_v4(),
            email,
            password: password_hash,
            permissions,
        }
    }
}