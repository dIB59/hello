// disable the dead code warning
#![allow(dead_code)]

use std::env;
use std::sync::atomic::{AtomicU32, Ordering};

use diesel::{Connection, PgConnection, RunQueryDsl, sql_query};
use diesel::backend::Backend;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::warn;

static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);
const TEST_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct TestDb {
    default_db_url: String,
    url: String,
    name: String,
    delete_on_drop: bool,
}

impl TestDb {
    pub fn new() -> Self {
        // Generate a unique database name
        let name = format!(
            "test_db_{}_{}",
            std::process::id(),
            TEST_DB_COUNTER.fetch_add(1, Ordering::SeqCst)
        );

        let default_db_url = "postgres://myuser:mypassword@localhost/postgres";

        let mut conn = PgConnection::establish(&default_db_url).unwrap();

        let sql = sql_query(format!("CREATE DATABASE \"{}\";", name))
            .execute(&mut conn)
            .expect("Failed to create test database");
        log::trace!("SQL IS: {:?}", sql);
        log::trace!("{}", &name);

        let url = format!("postgres://myuser:mypassword@localhost/{}", name);
        let mut conn = PgConnection::establish(&url).expect("Failed to connect to test database");
        log::info!("DATABASE URL IS: {:?}", url);
        run_test_migrations(&mut conn);

        Self {
            default_db_url: default_db_url.to_string(),
            url: url.to_string(),
            name,
            delete_on_drop: true,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn leak(&mut self) {
        self.delete_on_drop = false;
    }

    pub fn conn(&self) -> PgConnection {
        PgConnection::establish(&self.url.as_str()).expect("Failed to connect to test database")
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        if !self.delete_on_drop {
            warn!("TestDb leaking database {}", self.name);
            return;
        }

        let mut conn = PgConnection::establish(&self.default_db_url)
            .expect("Failed to connect to default database");

        let terminate_result = sql_query(format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';",
            self.name
        ))
        .execute(&mut conn);

        if terminate_result.is_err() {
            warn!(
                "Failed to terminate connections to the test database: {}",
                self.name
            );
        }

        // Attempt to drop the test database
        let drop_result =
            sql_query(format!("DROP DATABASE IF EXISTS \"{}\";", self.name)).execute(&mut conn); // Should not require a mutable reference

        if drop_result.is_err() {
            warn!("Failed to drop test database: {}", self.name);
        }
    }
}

pub fn run_test_migrations<DB: Backend>(connection: &mut impl MigrationHarness<DB>) {
    connection
        .run_pending_migrations(TEST_MIGRATIONS)
        .expect("Failed to run migrations");
}

fn migration_connection() -> diesel::PgConnection {
    let connection_url = database_url_from_env("DATABASE_URL");
    let mut conn = diesel::PgConnection::establish(&connection_url).unwrap();
    conn.begin_test_transaction().unwrap();
    conn
}

fn database_url_from_env(env_var: &str) -> String {
    env::var(env_var).expect(&format!("{} must be set", env_var))
}
