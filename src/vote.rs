use crate::{app::AppState, database::Song, html, page::page, view::View};
use axum::extract::{Path, State};
use random_string::{charsets, generate};
use std::sync::Arc;
use tracing::warn;

pub async fn vote_songs(State(state): State<Arc<AppState>>) -> View {
    let username = generate(6, charsets::ALPHA);

    let songs = state
        .database
        .get_setlist()
        .await
        .unwrap()
        .into_iter()
        .map(|x| song_card(x, &username))
        .collect::<View>();

    let song_container = html! { <div class="flex flex-col gap-3">{songs}</div> };

    page(song_container, "Setlist")
}

pub async fn vote_for_song(
    State(state): State<Arc<AppState>>,
    Path((username, song_id)): Path<(String, i32)>,
) -> View {
    warn!("New vote for song {} by {}", song_id, username);

    state
        .database
        .create_vote(&username, song_id)
        .await
        .unwrap();

    html! { <p>hej</p> }
}

fn song_card(song: Song, username: &str) -> View {
    html! {
        <div
            id=format!("song-{}", song.id)
            class="flex flex-col gap-1 p-4 max-w-lg rounded-lg border shadow dark:border-neutral-700 dark:bg-neutral-950"
        >

            <button hx-post=format!("/vote/{}/{}", username, song.id)>Vote</button>

            <div class="flex flex-wrap gap-2 justify-between items-center">
                <h2 class="text-2xl font-semibold whitespace-nowrap">{song.title}</h2>

            </div>
            <h4 class="text-sm text-neutral-500">{song.artist}</h4>
            {if let Some(description) = song.description {
                html! { <p class="pt-2">{description}</p> }
            } else {
                Default::default()
            }}
        </div>
    }
}
