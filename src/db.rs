use sqlx::{postgres::PgPoolOptions, Postgres, Pool};


#[derive(Clone, Debug)]
pub struct Database {
    pub connection_pool: Pool<Postgres>,
}

impl Database {

    pub async fn create_connection() -> Self {
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(err) => {
                tracing::error!("Cannot find DATABASE_URL variable {}", err);
                panic!();
            }
        };

        let pool = match PgPoolOptions::new().connect(&database_url).await {
            Ok(conn) => conn,
            Err(err) => {
                tracing::error!("Cannot connect to database  {}", err);
                panic!();
            }
        };

        Self {
            connection_pool: pool,
        }
    }

}