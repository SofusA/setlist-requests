CREATE TABLE IF NOT EXISTS songs (
    id serial primary key not null unique, 
    artist text not null,
    title text not null,
    description text
);

CREATE TABLE IF NOT EXISTS votes (
    id serial PRIMARY KEY NOT NULL UNIQUE, 
    session_id text NOT NULL,
    song_id int NOT NULL,
    CONSTRAINT unique_session_song UNIQUE (session_id, song_id),
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE
);
