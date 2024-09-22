use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use tracing::{info, warn};

impl Database {
    pub async fn new(credentials: Credentials) -> Database {
        let database_url = format!(
            "postgresql://{}:{}@{}:5432/{}",
            credentials.user, credentials.secret, credentials.hostname, credentials.database
        );

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();

        match sqlx::migrate!("./migrations").run(&pool).await {
            Ok(_) => info!("Migrated database"),
            Err(_) => warn!("Error migrating database"),
        };

        Database { pool }
    }

    pub fn new_from_pool(pool: PgPool) -> Database {
        Database { pool }
    }

    pub async fn get_setlist(&self) -> Result<Vec<Song>> {
        let result = sqlx::query_as!(Song, "select * from songs")
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn get_votes(&self) -> Result<Vec<Vote>> {
        let result = sqlx::query_as!(Vote, "select * from votes")
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn create_vote(&self, username: &str, song_id: i32) -> Result<()> {
        sqlx::query!(
            "insert into votes values (default, $1, $2)",
            username,
            song_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn clear_votes(&self) -> Result<()> {
        sqlx::query!("truncate table votes")
            .fetch_all(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn add_song(
        &self,
        artist: &str,
        title: &str,
        description: Option<&str>,
    ) -> Result<()> {
        sqlx::query!(
            "insert into songs values (default, $1, $2, $3)",
            artist,
            title,
            description
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_song(&self, id: i32) -> Result<()> {
        sqlx::query!("delete from songs where id = $1", id)
            .fetch_all(&self.pool)
            .await?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct Credentials {
    pub hostname: String,
    pub secret: String,
    pub user: String,
    pub database: String,
}

pub struct Database {
    pool: Pool<Postgres>,
}

#[derive(sqlx::FromRow)]
pub struct Song {
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct Vote {
    pub id: i32,
    pub username: String,
    pub song_id: i32,
}
