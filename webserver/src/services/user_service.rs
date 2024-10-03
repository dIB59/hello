use diesel::prelude::*;
use diesel::result::Error;

use crate::models::user::{NewUser, User};
use crate::schema::users;

pub async fn register_user(
    conn: &mut PgConnection,
    username: &str,
    password: &str,
    email: &str,
) -> Result<User, Error> {
    let password_hash = &hash_password(password).expect("Failed to hash password");
    let new_user = NewUser {
        username,
        password_hash,
        email,
    };

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn);
    log::info!("{:?}", user);
    return user;
}

pub(crate) fn get_user_by_id(user_id: i32, conn: &mut PgConnection) -> Result<User, Error> {
    users::table.find(user_id).first(conn)
}

pub(crate) fn get_user_by_email(email: &str, conn: &mut PgConnection) -> Result<User, Error> {
    users::table.filter(users::email.eq(email)).first(conn)
}

pub(crate) fn get_user_id_by_email(email: &str, conn: &mut PgConnection) -> Result<i32, Error> {
    users::table
        .filter(users::email.eq(email))
        .select(users::id)
        .first(conn)
}

pub fn login(conn: &mut PgConnection, email: &str, password: &str) -> Result<User, Error> {
    let user = get_user_by_email(email, conn);

    match user {
        Ok(user) => {
            let is_password_correct = verify_password(&user.password_hash, password)
                .expect("Password verification failed");
            return if is_password_correct {
                Ok(user)
            } else {
                Err(Error::NotFound)
            };
        }
        Err(error) => Err(error),
    }
}

fn hash_password(plain: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(plain, bcrypt::DEFAULT_COST)
}

fn verify_password(hash: &str, plain: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(plain, hash)
}

#[cfg(test)]
mod tests {
    use crate::database::test_db::TestDb;

    use super::*;

    #[actix_rt::test]
    async fn test_register_user_success() {
        let db = TestDb::new();
        let mut conn = db.conn();

        let username = "testuser";
        let password = "password123";
        let email = "test@example.com";

        let result = register_user(&mut conn, username, password, email).await;
        println!("{:?}", result);
        assert!(
            result.is_ok(),
            "User registration failed when it should have succeeded"
        );

        let registered_user = result.unwrap();
        assert_eq!(registered_user.username, username);

        // Ensure the password is hashed
        let is_password_correct = verify_password(&registered_user.password_hash, password)
            .expect("Password verification failed");
        assert!(
            is_password_correct,
            "Password hashing or verification failed"
        );
    }

    #[actix_rt::test]
    async fn test_register_user_duplicate_username() {
        let db = TestDb::new();
        let mut conn = db.conn();

        let username = "testuser";
        let password = "password123";
        let email = "test@example.com";
        // First registration should succeed

        let result = register_user(&mut conn, username, password, email).await;
        println!("{:?}", result);
        assert!(
            result.is_ok(),
            "User registration failed when it should have succeeded"
        );

        // Second registration with the same username should fail
        let second_result = register_user(&mut conn, username, password, email).await;
        assert!(
            second_result.is_err(),
            "Second user registration succeeded when it should have failed due to duplicate username"
        );
    }

    #[actix_rt::test]
    async fn test_password_hashing() {
        let password = "password123";
        let hashed_password = hash_password(password).expect("Failed to hash password");

        assert_ne!(password, hashed_password);
    }

    #[actix_rt::test]
    async fn test_verify_password() {
        let password = "password123";
        let hashed_password = hash_password(password).expect("Failed to hash password");

        let is_password_correct =
            verify_password(&hashed_password, password).expect("Password verification failed");
        assert!(is_password_correct, "Password verification failed");
    }
}
