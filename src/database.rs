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

    pub async fn get_vote_results(&self) -> Result<Vec<VoteResult>> {
        let rows: Vec<SongWithVotes> = sqlx::query_as!(
            SongWithVotes,
            r#"
        SELECT 
            s.id, 
            s.artist, 
            s.title, 
            s.description, 
            COUNT(v.id) AS vote_count
        FROM 
            songs s
        LEFT JOIN 
            votes v ON s.id = v.song_id
        GROUP BY 
            s.id
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut result: Vec<_> = rows
            .into_iter()
            .map(|row| VoteResult {
                song: Song {
                    id: row.id,
                    artist: row.artist,
                    title: row.title,
                    description: row.description,
                },
                vote_count: row.vote_count.unwrap_or(0),
            })
            .collect();

        result.sort_unstable_by(|a, b| b.vote_count.cmp(&a.vote_count));

        Ok(result)
    }

    pub async fn get_votes(&self, session_id: &str) -> Result<Vec<Vote>> {
        let result = sqlx::query_as!(
            Vote,
            "select * from votes where session_id = $1",
            session_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn create_vote(&self, username: &str, song_id: i32) -> Result<Song> {
        let result = sqlx::query_as!(
            Song,
            "with inserted_vote as (insert into votes values (default ,$1, $2) on conflict (session_id, song_id) do nothing returning song_id) select s.* from inserted_vote iv join songs s on iv.song_id = s.id",
            username,
            song_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn delete_vote(&self, username: &str, song_id: i32) -> Result<Song> {
        let result = sqlx::query_as!(
            Song,
            "with deleted_vote as (delete from votes where session_id = $1 and song_id = $2 returning song_id) select s.* from deleted_vote dv join songs s on dv.song_id = s.id",
            username,
            song_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
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
    ) -> Result<Song> {
        let result = sqlx::query_as!(
            Song,
            "insert into songs values (default, $1, $2, $3) returning *",
            artist,
            title,
            description
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
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
    pub session_id: String,
    pub song_id: i32,
}

#[derive(sqlx::FromRow)]
struct SongWithVotes {
    id: i32,
    artist: String,
    title: String,
    description: Option<String>,
    vote_count: Option<i64>,
}

pub struct VoteResult {
    pub song: Song,
    pub vote_count: i64,
}
