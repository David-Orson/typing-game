use actix_web::web::Data;
use std::io;

use crate::{store::models::test::Test, AppState};

pub async fn get_all_by_account(
    id: u32,
    state: &Data<AppState>,
) -> Result<Vec<Test>, io::Error> {
    match sqlx::query_as::<_, Test>(
        "SELECT * FROM test WHERE account_id = $1",
    )
    .bind(i32::try_from(id).ok())
    .fetch_all(&state.db)
    .await
    {
        Ok(tests) => return Ok(tests),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                e,
            ))
        }
    }
}

pub async fn create(
    test: Test,
    state: &Data<AppState>,
) -> Result<String, io::Error> {
    match sqlx::query(
        "INSERT INTO test (
            account_id, 
            test, 
            typed,
            wpm,
            accuracy
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5
        )",
    )
    .bind(test.account_id)
    .bind(test.test)
    .bind(test.typed)
    .bind(test.wpm)
    .bind(test.accuracy)
    .execute(&state.db)
    .await
    {
        Ok(_) => return Ok(String::from("Added test")),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                e,
            ))
        }
    }
}
