use crate::{app::AppState, database::Song, html, icons, page::page, view::View};
use axum::{
    extract::{Path, State},
    Form,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::warn;

#[derive(Deserialize, Debug)]
pub struct CreateSongInput {
    title: String,
    artist: String,
    description: String,
}

pub async fn add_song(
    State(state): State<Arc<AppState>>,
    Form(input): Form<CreateSongInput>,
) -> View {
    let description = match input.description.is_empty() {
        true => None,
        false => Some(input.description.as_str()),
    };

    let song = state
        .database
        .add_song(&input.artist, &input.title, description)
        .await
        .unwrap();

    song_card(song)
}

pub async fn delete_song(Path(id): Path<i32>, State(state): State<Arc<AppState>>) {
    warn!("Deleting song {}", id);
    state.database.delete_song(id).await.unwrap();
}

pub async fn hide_song(Path(id): Path<i32>, State(state): State<Arc<AppState>>) -> View {
    warn!("Hiding song {}", id);
    let song = state.database.hide_song(id).await.unwrap();

    song_card(song)
}

pub async fn unhide_song(Path(id): Path<i32>, State(state): State<Arc<AppState>>) -> View {
    warn!("Unhiding song {}", id);
    let song = state.database.unhide_song(id).await.unwrap();

    song_card(song)
}

fn song_card(song: Song) -> View {
    html! {
        <div
            id=format!("song-{}", song.id)
            class=format!(
                "flex flex-col gap-1 p-4 rounded-lg border shadow dark:border-neutral-700 dark:bg-neutral-950 transition-colors {}",
                if song.hidden { "text-neutral-500" } else { Default::default() },
            )
        >
            <div class="flex flex-wrap gap-2 justify-between items-center">
                <h2 class="text-2xl font-semibold whitespace-nowrap">{song.title}</h2>

                <div class="flex gap-4">
                    {if song.hidden {
                        html! {
                            <button
                                title="Unhide song"
                                hx-put=format!("/setlist/{}/unhide", song.id)
                                hx-target=format!("#song-{}", song.id)
                                hx-swap="outerHTML"
                            >
                                {icons::eye()}
                            </button>
                        }
                    } else {
                        html! {
                            <button
                                title="Hide song"
                                hx-put=format!("/setlist/{}/hide", song.id)
                                hx-target=format!("#song-{}", song.id)
                                hx-swap="outerHTML"
                            >
                                {icons::eye_off()}
                            </button>
                        }
                    }}
                    <button
                        hx-delete=format!("/setlist/{}", song.id)
                        hx-target=format!("#song-{}", song.id)
                        hx-swap="outerHTML"
                        class="text-red-500"
                    >
                        {icons::trash_2()}
                    </button>
                </div>
            </div>
            <h4 class="text-sm">{song.artist}</h4>
            {if let Some(description) = song.description {
                html! { <p class="pt-2">{description}</p> }
            } else {
                Default::default()
            }}
        </div>
    }
}

pub async fn clear_votes(State(state): State<Arc<AppState>>) {
    state.database.clear_votes().await.unwrap();
}

pub async fn setlist_page(State(state): State<Arc<AppState>>) -> View {
    let songs = state
        .database
        .get_setlist()
        .await
        .unwrap()
        .into_iter()
        .map(song_card)
        .collect::<View>();

    let song_container = html! {
        <div class="flex flex-col gap-3 w-full max-w-lg">
            <button
                hx-post="/setlist/votes/clear"
                hx-swap="none"
                class="flex gap-3 justify-center p-4 max-w-lg rounded-lg border shadow transition-colors dark:border-neutral-700 dark:bg-neutral-950"
            >
                <p>Clear votes</p>
                <p class="text-red-500">{icons::trash_2()}</p>
            </button>
            {songs}
            <details id="add-song">
                <form
                    id="add-song-form"
                    hx-post="/setlist"
                    hx-target="#add-song"
                    hx-swap="beforebegin"
                    class="flex flex-col gap-3 p-4 mt-3 max-w-lg rounded-lg border shadow text-neutral-500 dark:border-neutral-700 dark:bg-neutral-950"
                >
                    <label class="dark:text-white" for="title">
                        {"Title:"}
                    </label>
                    <input class="p-1 rounded bg-neutral-300" type="text" id="title" name="title" />
                    <label class="dark:text-white" for="artist">

                        {"Artist:"}

                    </label>
                    <input
                        class="p-1 rounded bg-neutral-300"
                        type="text"
                        id="artist"
                        name="artist"
                    />
                    <label class="dark:text-white" for="description">
                        {"Description:"}
                    </label>
                    <textarea
                        class="p-1 rounded bg-neutral-300"
                        id="description"
                        name="description"
                    ></textarea>

                    <input
                        class="p-1 text-white bg-blue-500 rounded transition-colors hover:bg-blue-400"
                        type="submit"
                        value="Submit"
                    />
                </form>
                <summary class="flex flex-col items-center p-4 max-w-lg rounded-lg border shadow transition-colors cursor-pointer hover:text-white text-neutral-500 dark:border-neutral-700 dark:bg-neutral-950">
                    {icons::plus_circle()}
                </summary>
            </details>
        </div>
    };

    page(song_container, "Setlist")
}
