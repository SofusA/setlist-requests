use crate::{app::AppState, database::Song, html, icons, page::page, view::View};
use axum::extract::{Path, State};
use axum_extra::extract::CookieJar;
use std::sync::Arc;
use tracing::warn;

pub async fn vote_songs(State(state): State<Arc<AppState>>, jar: CookieJar) -> View {
    let session_id = jar.get("session_id").unwrap().value_trimmed();

    let votes: Vec<i32> = state
        .database
        .get_votes(session_id)
        .await
        .unwrap()
        .iter()
        .map(|x| x.song_id)
        .collect();

    let songs = state
        .database
        .get_setlist()
        .await
        .unwrap()
        .into_iter()
        .map(|x| song_card(votes.contains(&x.id), x))
        .collect::<View>();

    let song_container = html! { <div class="flex flex-col gap-3 min-w-96">{songs}</div> };

    page(song_container, "Setlist")
}

pub async fn vote_for_song(
    State(state): State<Arc<AppState>>,
    Path(song_id): Path<i32>,
    jar: CookieJar,
) -> View {
    let session_id = jar.get("session_id").unwrap().value_trimmed();

    warn!("New vote for song {} by {}", song_id, session_id);

    let song = state
        .database
        .create_vote(session_id, song_id)
        .await
        .unwrap();

    song_card(true, song)
}

pub async fn delete_vote(
    State(state): State<Arc<AppState>>,
    Path(song_id): Path<i32>,
    jar: CookieJar,
) -> View {
    let session_id = jar.get("session_id").unwrap().value_trimmed();

    warn!("Delete vote for song {} by {}", song_id, session_id);

    let song = state
        .database
        .delete_vote(session_id, song_id)
        .await
        .unwrap();

    song_card(false, song)
}

fn song_card(voted_for: bool, song: Song) -> View {
    html! {
        <button
            {if voted_for {
                format!("hx-delete=/vote/{}", song.id)
            } else {
                format!("hx-post=/vote/{}", song.id)
            }}
            hx-swap="outerHTML"
            id=format!("song-{}", song.id)
            class=format!(
                "flex transition-all flex-col gap-1 p-4 max-w-lg rounded-lg border shadow  dark:bg-neutral-950 items-start {}",
                if voted_for { "border-blue-500" } else { "dark:border-neutral-700" },
            )
        >
            <div class="flex flex-wrap gap-2 justify-between items-center w-full">
                <h2 class="text-2xl font-semibold whitespace-nowrap">{song.title}</h2>
                <span class=if voted_for {
                    "text-blue-500"
                } else {
                    "dark:text-neutral-700"
                }>{icons::check_circle()}</span>
            </div>

            <h4 class="text-sm text-neutral-500">{song.artist}</h4>
            {if let Some(description) = song.description {
                html! { <p class="pt-2">{description}</p> }
            } else {
                Default::default()
            }}
        </button>
    }
}
