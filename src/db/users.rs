use diesel::prelude::*;
use chrono::Utc;
use crate::models::{NewUser, User};
use crate::errors::ServiceError;
use log::error;

pub fn create_user(pool: &crate::db::DbPool, new_user: NewUser) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::*;
    
    let conn = &mut pool.get().map_err(|e| {
        ServiceError::DatabaseError(format!("Connection error: {}", e))
    })?;
    
    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .map_err(|error| {
            error!("Failed to create user: {:?}", error);
            // Convert the diesel error into our ServiceError
            match error {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _
                ) => ServiceError::Conflict(format!(
                    "User with username '{}' or email '{}' already exists",
                    new_user.username,
                    new_user.email
                )),
                _ => ServiceError::DatabaseError(error.to_string())
            }
        })
}

pub fn get_user_by_id(pool: &crate::db::DbPool, user_id: i32) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::*;
    
    let conn = &mut pool.get().map_err(|e| {
        ServiceError::DatabaseError(format!("Connection error: {}", e))
    })?;
    
    users
        .find(user_id)
        .first(conn)
        .map_err(|error| {
            error!("Failed to get user by id: {:?}", error);
            ServiceError::DatabaseError(error.to_string())
        })
}

pub fn get_user_by_email(pool: &crate::db::DbPool, user_email: &str) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::*;
    
    let conn = &mut pool.get().map_err(|e| {
        ServiceError::DatabaseError(format!("Connection error: {}", e))
    })?;
    
    users
        .filter(email.eq(user_email))
        .first(conn)
        .map_err(|error| {
            error!("Failed to get user by email: {:?}", error);
            ServiceError::DatabaseError(error.to_string())
        })
}

pub fn update_user(pool: &crate::db::DbPool, user_id: i32, updated_user: NewUser) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::*;
    
    let conn = &mut pool.get().map_err(|e| {
        ServiceError::DatabaseError(format!("Connection error: {}", e))
    })?;
    
    diesel::update(users.find(user_id))
        .set((
            username.eq(updated_user.username),
            email.eq(updated_user.email),
            password_hash.eq(updated_user.password_hash),
            updated_at.eq(Utc::now().naive_utc()),
        ))
        .get_result(conn)
        .map_err(|error| {
            error!("Failed to update user: {:?}", error);
            ServiceError::DatabaseError(error.to_string())
        })
}

pub fn delete_user(pool: &crate::db::DbPool, user_id: i32) -> Result<(), ServiceError> {
    use crate::schema::users::dsl::*;
    
    let conn = &mut pool.get().map_err(|e| {
        ServiceError::DatabaseError(format!("Connection error: {}", e))
    })?;
    
    diesel::delete(users.find(user_id))
        .execute(conn)
        .map_err(|error| {
            error!("Failed to delete user: {:?}", error);
            ServiceError::DatabaseError(error.to_string())
        })?;
    
    Ok(())
}
