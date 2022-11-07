use sqlx::{query, Pool, Postgres};

const MIGRATIONS: [&str; 1] = ["--sql
    CREATE TABLE IF NOT EXISTS account (
        id serial,
        username varchar(30) NOT NULL DEFAULT '',
        email varchar(50) NOT NULL,
        password varchar(64) NOT NULL,
        create_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        modify_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY (id)
    );"];

pub async fn up(pool: &Pool<Postgres>) {
    for s in MIGRATIONS.iter() {
        match query::<Postgres>(s).execute(pool).await {
            Err(e) => println!("{:?}", e),
            _ => (),
        };
    }
}
