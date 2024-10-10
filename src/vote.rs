use crate::{
    app::AppState, database::Song, errors::BadRequestError, html, icons, page::page, view::View,
    vote_results::votes_updated,
};
use axum::{
    extract::{Path, State},
    response::Redirect,
};
use axum_extra::extract::CookieJar;
use std::sync::Arc;
use tracing::warn;

const MAX_VOTES: i64 = 5;

pub async fn vote_songs(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<View, Redirect> {
    let session_id = match jar.get("session_id") {
        Some(res) => res.value_trimmed(),
        None => return Err(Redirect::to("/")),
    };

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
        .filter(|x| !x.hidden)
        .map(|x| song_card(votes.contains(&x.id), x))
        .collect::<View>();

    let current_votes = state.database.count_votes(session_id).await.unwrap();

    let song_container = html! {
        <div class="flex flex-col gap-3 w-full max-w-lg">
            <div>
                <span id="current_votes">{current_votes}</span>
                {"&nbsp;"}
                <span>ud af</span>
                {"&nbsp;"}
                <span>{MAX_VOTES}</span>
                {"&nbsp;"}
                <span>stemmer</span>
            </div>

            <div class="flex flex-col gap-3">{songs}</div>

            <a
                href="/"
                class="flex justify-center py-2 px-3 text-white bg-blue-500 rounded hover:bg-blue-400"
            >
                Afslut
            </a>
        </div>
    };

    Ok(page(song_container, "Setlist"))
}

pub async fn vote_for_song(
    State(state): State<Arc<AppState>>,
    Path(song_id): Path<i32>,
    jar: CookieJar,
) -> Result<View, BadRequestError> {
    let session_id = jar.get("session_id").unwrap().value_trimmed();

    warn!("New vote for song {} by {}", song_id, session_id);

    let vote_count = state.database.count_votes(session_id).await.unwrap();

    if vote_count >= MAX_VOTES {
        return Err(BadRequestError::TooManyVotes);
    }

    let song = state
        .database
        .create_vote(session_id, song_id)
        .await
        .unwrap();

    votes_updated(&state.tx, &state.database).await;

    Ok(html! {
        {song_card(true, song)}
        <span id="current_votes" hx-swap-oob="true">
            {vote_count + 1}
        </span>
    })
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

    votes_updated(&state.tx, &state.database).await;

    let vote_count = state.database.count_votes(session_id).await.unwrap();

    html! {
        {song_card(false, song)}
        <span id="current_votes" hx-swap-oob="true">
            {vote_count}
        </span>
    }
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
                "flex transition-all flex-col gap-1 p-4 w-full rounded-lg border shadow items-start {}",
                if voted_for { "border-blue-500" } else { "dark:border-neutral-700" },
            )
        >
            <div class="flex flex-wrap gap-2 justify-between items-center w-full">
                <h2 class="text-2xl font-semibold">{song.title}</h2>
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
