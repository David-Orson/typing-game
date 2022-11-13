use actix_web::web::Data;
use std::io;

use crate::{store::models::test::Test, AppState};

pub async fn create(
    test: Test,
    state: &Data<AppState>,
) -> Result<String, io::Error> {
    match sqlx::query(
        "INSERT INTO test (
            account_id, 
            test, 
            typed
        ) VALUES (
            $1,
            $2,
            $3
        )",
    )
    .bind(test.account)
    .bind(test.test)
    .bind(test.typed)
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
