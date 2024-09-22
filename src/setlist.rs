use crate::{app::AppState, database::Song, html, page::page, view::View};
use axum::{extract::State, Form};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct Input {
    title: String,
    artist: String,
    description: String,
}

pub async fn add_song(State(state): State<Arc<AppState>>, Form(input): Form<Input>) -> View {
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

fn song_card(song: Song) -> View {
    html! {
        <div class="flex flex-col gap-1 p-4 max-w-lg rounded-lg border shadow dark:border-neutral-700 dark:bg-neutral-950">
            <h2 class="text-2xl font-semibold whitespace-nowrap">{song.title}</h2>
            <h4 class="text-sm text-neutral-500">{song.artist}</h4>
            {if let Some(description) = song.description {
                html! { <p class="pt-2">{description}</p> }
            } else {
                Default::default()
            }}
        </div>
    }
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
        <div class="flex flex-col gap-3">
            {songs}
            <form
                id="add-song-form"
                hx-post="/setlist"
                hx-swap="beforebegin"
                class="hidden flex-col gap-3 p-4 max-w-lg rounded-lg border shadow text-neutral-500 dark:border-neutral-700 dark:bg-neutral-950"
            >
                <label class="dark:text-white" for="title">
                    {"Title:"}
                </label>
                <input class="p-1 rounded bg-neutral-300" type="text" id="title" name="title" />
                <label class="dark:text-white" for="artist">

                    {"Artist:"}

                </label>
                <input class="p-1 rounded bg-neutral-300" type="text" id="artist" name="artist" />
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
            <div class="flex flex-col items-center p-4 max-w-lg rounded-lg border shadow transition-colors cursor-pointer hover:text-white text-neutral-500 dark:border-neutral-700 dark:bg-neutral-950">
                <i data-feather="plus-circle"></i>
                <script>feather.replace();</script>
                <script>
                    me().on("click", ev => {
                        me("#add-song-form").classToggle("hidden");
                        me("#add-song-form").classToggle("flex");
                    })
                </script>
            </div>
        </div>
    };

    page(song_container, "Setlist")
}
