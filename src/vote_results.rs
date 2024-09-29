use crate::{app::AppState, html, page::page, view::View};
use axum::extract::State;
use std::sync::Arc;

pub async fn vote_result_page(State(state): State<Arc<AppState>>) -> View {
    let votes = state
        .database
        .get_vote_results()
        .await
        .unwrap()
        .iter()
        .map(|vote| {
            html! {
                <tr class="odd:bg-gray-50 odd:dark:bg-neutral-950">
                    <td class="py-3 px-6">{&vote.song.title}</td>
                    <td class="py-3 px-6">{&vote.song.artist}</td>
                    <td class="py-3 px-6">{&vote.vote_count}</td>
                </tr>
            }
        })
        .collect::<View>();

    let results = html! {
        <div class="w-full max-w-lg rounded-lg border border-neutral-700 overflow-clip">
            <table class="w-full text-left table-auto">
                <tr class="font-bold border-b border-gray-700 bg-neutral-950">
                    <th class="py-3 px-6">Title</th>
                    <th class="py-3 px-6">Artist</th>
                    <th class="py-3 px-6">Votes</th>
                </tr>
                {votes}
            </table>
        </div>
    };

    page(results, "Vote results")
}
