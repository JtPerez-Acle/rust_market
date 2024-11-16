#[path = "test_logging.rs"]
mod test_logging;
#[path = "test_config.rs"]
mod test_config;

use test_logging::TestLogger;
use test_config::setup_test_db;
use rust_market::models::{NewUser, User};
use rust_market::schema::users;
use diesel::prelude::*;
use rust_market::db::establish_connection_pool;
use uuid::Uuid;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use futures::future::join_all;
use std::time::Instant;

type BoxError = Box<dyn Error + Send + Sync + 'static>;

fn setup() {
    setup_test_db();
}

async fn create_test_user(
    conn: &mut PgConnection,
    username: String,
    email: String,
) -> Result<User, BoxError> {
    let new_user = NewUser::new(
        username,
        email,
        "test_hash".to_string(),
    );

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(conn)
        .map_err(|e| Box::new(e) as BoxError)
}

#[tokio::test]
async fn test_user_crud_operations() -> Result<(), BoxError> {
    setup();
    let logger = Arc::new(Mutex::new(TestLogger::new(
        "user_crud",
        "Test basic user CRUD operations"
    )));
    let pool = establish_connection_pool(None)?;
    let conn = &mut pool.get()?;

    // Test 1: Create User
    let start = Instant::now();
    let username = format!("test_user_{}", Uuid::new_v4());
    let email = format!("test{}@example.com", Uuid::new_v4());
    let user = create_test_user(conn, username.clone(), email.clone()).await?;
    let duration = start.elapsed();
    {
        let mut logger = logger.lock().unwrap();
        logger.log_operation("create_user", duration);
    }

    // Test 2: Read User
    let start = Instant::now();
    let found_user = users::table
        .find(user.id)
        .first::<User>(conn)
        .map_err(|e| Box::new(e) as BoxError)?;
    let duration = start.elapsed();
    {
        let mut logger = logger.lock().unwrap();
        logger.log_operation("read_user", duration);
    }
    assert_eq!(found_user.username, username);

    // Test 3: Update User
    let start = Instant::now();
    let new_username = format!("updated_user_{}", Uuid::new_v4());
    let updated_user = diesel::update(users::table.find(user.id))
        .set(users::username.eq(&new_username))
        .get_result::<User>(conn)
        .map_err(|e| Box::new(e) as BoxError)?;
    let duration = start.elapsed();
    {
        let mut logger = logger.lock().unwrap();
        logger.log_operation("update_user", duration);
    }
    assert_eq!(updated_user.username, new_username);

    // Test 4: Delete User
    let start = Instant::now();
    diesel::delete(users::table.find(user.id))
        .execute(conn)
        .map_err(|e| Box::new(e) as BoxError)?;
    let duration = start.elapsed();
    {
        let mut logger = logger.lock().unwrap();
        logger.log_operation("delete_user", duration);
    }

    {
        let mut logger = logger.lock().unwrap();
        logger.finish(true, None);
    }
    Ok(())
}

#[tokio::test]
async fn test_concurrent_user_operations() -> Result<(), BoxError> {
    setup();
    let logger = Arc::new(Mutex::new(TestLogger::new(
        "concurrent_users",
        "Test concurrent user operations"
    )));
    let pool = establish_connection_pool(None)?;

    let mut handles = vec![];
    let num_users = 10;

    for i in 0..num_users {
        let pool = pool.clone();
        let logger = Arc::clone(&logger);

        handles.push(tokio::spawn(async move {
            let conn = &mut pool.get()?;
            let start = Instant::now();
            let username = format!("concurrent_user_{}_{}", i, Uuid::new_v4());
            let email = format!("concurrent{}@example.com", i);
            let result = create_test_user(conn, username, email).await;
            let duration = start.elapsed();
            
            {
                let mut logger = logger.lock().unwrap();
                logger.log_operation(&format!("create_concurrent_user_{}", i), duration);
                if let Err(e) = &result {
                    logger.log_error(&format!("create_concurrent_user_{}", i), &e.to_string());
                }
            }
            
            result
        }));
    }

    let results = join_all(handles).await;
    let mut created_users = Vec::new();

    for result in results {
        match result {
            Ok(Ok(user)) => created_users.push(user),
            Ok(Err(e)) => {
                {
                    let mut logger = logger.lock().unwrap();
                    logger.log_error("concurrent_user_creation", &e.to_string());
                }
                return Err(e);
            }
            Err(e) => {
                {
                    let mut logger = logger.lock().unwrap();
                    logger.log_error("concurrent_user_creation", &e.to_string());
                }
                return Err(Box::new(e));
            }
        }
    }

    // Cleanup: Delete created users
    let conn = &mut pool.get()?;
    for user in created_users {
        if let Err(e) = diesel::delete(users::table.find(user.id))
            .execute(conn)
            .map_err(|e| Box::new(e) as BoxError)
        {
            {
                let mut logger = logger.lock().unwrap();
                logger.log_error("cleanup_users", &e.to_string());
            }
            return Err(e);
        }
    }

    {
        let mut logger = logger.lock().unwrap();
        logger.finish(true, None);
    }
    Ok(())
}

#[tokio::test]
async fn test_user_edge_cases() -> Result<(), BoxError> {
    setup();
    let logger = Arc::new(Mutex::new(TestLogger::new(
        "user_edge_cases",
        "Test edge cases in user operations"
    )));
    let pool = establish_connection_pool(None)?;
    let conn = &mut pool.get()?;

    // Test 1: Create user with duplicate email
    let start = Instant::now();
    let email = format!("duplicate{}@example.com", Uuid::new_v4());
    let _ = create_test_user(
        conn,
        format!("test_user_{}", Uuid::new_v4()),
        email.clone(),
    ).await?;

    let result = create_test_user(
        conn,
        format!("test_user_{}", Uuid::new_v4()),
        email,
    ).await;
    let duration = start.elapsed();
    
    {
        let mut logger = logger.lock().unwrap();
        logger.log_operation("duplicate_email", duration);
        if let Err(e) = &result {
            logger.log_error("duplicate_email", &e.to_string());
        }
    }

    // Test 2: Create user with empty username
    let start = Instant::now();
    let result = create_test_user(
        conn,
        "".to_string(),
        format!("empty{}@example.com", Uuid::new_v4()),
    ).await;
    let duration = start.elapsed();
    
    {
        let mut logger = logger.lock().unwrap();
        logger.log_operation("empty_username", duration);
        if let Err(e) = &result {
            logger.log_error("empty_username", &e.to_string());
        }
    }

    // Test 3: Create user with invalid email
    let start = Instant::now();
    let result = create_test_user(
        conn,
        format!("test_user_{}", Uuid::new_v4()),
        "invalid_email".to_string(),
    ).await;
    let duration = start.elapsed();
    
    {
        let mut logger = logger.lock().unwrap();
        logger.log_operation("invalid_email", duration);
        if let Err(e) = &result {
            logger.log_error("invalid_email", &e.to_string());
        }
    }

    {
        let mut logger = logger.lock().unwrap();
        logger.finish(true, None);
    }
    Ok(())
}
