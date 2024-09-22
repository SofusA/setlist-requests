use crate::{app::AppState, html, page::page, view::View};
use axum::extract::State;
use std::sync::Arc;

pub async fn setlist_page(State(state): State<Arc<AppState>>) -> View {
    let songs = state
        .database
        .get_setlist()
        .await
        .unwrap()
        .into_iter()
        .map(|song| {
            html! {
                <div class="flex flex-col gap-1 p-4 max-w-lg rounded-lg border shadow dark:border-gray-700 dark:bg-gray-950">
                    <h2 class="text-2xl font-semibold whitespace-nowrap">{song.title}</h2>
                    <h4 class="text-sm text-gray-500">{song.artist}</h4>
                    {if let Some(description) = song.description {
                        html! { <p class="pt-2">{description}</p> }
                    } else {
                        Default::default()
                    }}
                </div>
            }
        })
        .collect::<View>();

    let song_container = html! { <div class="flex flex-col gap-3">{songs}</div> };

    page(song_container, "lol")
}
