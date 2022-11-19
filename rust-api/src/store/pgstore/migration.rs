use sqlx::{query, Pool, Postgres};

const MIGRATIONS: [&str; 3] = [
    "--sql
    CREATE TABLE IF NOT EXISTS account (
        id serial,
        username varchar(30) NOT NULL DEFAULT '',
        email varchar(50) NOT NULL,
        password varchar(64) NOT NULL,
        create_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        modify_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY (id)
    );",
    "CREATE TABLE IF NOT EXISTS token (
			id serial,
			account_id int NOT NULL,
			username varchar(30) NOT NULL DEFAULT '',
			token char(100) NOT NULL,
			create_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			modify_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			PRIMARY KEY (id),
			FOREIGN KEY (account_id) REFERENCES account(id)
		);",
    "--sql
    CREATE TABLE IF NOT EXISTS test (
        id serial,
        account_id int NOT NULL,
        test varchar(250) NOT NULL DEFAULT '',
        typed char(250) NOT NULL,
        wpm int NOT NULL,
        accuracy real NOT NULL,
        create_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        modify_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY (id),
        FOREIGN KEY (account_id) REFERENCES account(id)
    )",
];

pub async fn up(pool: &Pool<Postgres>) {
    for s in MIGRATIONS.iter() {
        match query::<Postgres>(s).execute(pool).await {
            Err(e) => println!("{:?}", e),
            _ => (),
        };
    }
}
