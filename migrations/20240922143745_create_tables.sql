CREATE TABLE IF NOT EXISTS songs (
    id serial primary key not null unique, 
    artist text not null,
    title text not null,
    description text
);

CREATE TABLE IF NOT EXISTS votes (
    id serial primary key not null unique, 
    username text not null,
    song_id int not null,
    FOREIGN KEY (song_id) REFERENCES songs(id)
);
