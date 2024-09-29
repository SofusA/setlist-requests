use crate::{app::AppState, database::Database, html, page::page, view::View};
use axum::extract::{ws::Message, State};
use std::sync::Arc;
use tokio::sync::watch::Sender;
use tracing::warn;

pub async fn votes_updated(tx: &Sender<Message>, database: &Database) {
    let vote_results = vote_results(database).await;

    if tx.send(Message::Text(vote_results.to_string())).is_err() {
        warn!("Failed to send message");
    }
}

async fn vote_results(database: &Database) -> View {
    let votes = database
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

    html! {
        <div
            id="vote-results"
            class="w-full max-w-lg rounded-lg border border-neutral-700 overflow-clip"
        >
            <table class="w-full text-left table-auto">
                <tr class="font-bold border-b border-gray-700 bg-neutral-950">
                    <th class="py-3 px-6">Title</th>
                    <th class="py-3 px-6">Artist</th>
                    <th class="py-3 px-6">Votes</th>
                </tr>
                {votes}
            </table>
        </div>
    }
}

pub async fn vote_result_page(State(state): State<Arc<AppState>>) -> View {
    let vote_results = vote_results(&state.database).await;

    page(
        html! {
            {vote_results}
            <script src="/assets/scripts/vote-updates.js?version=1"></script>
        },
        "Vote results",
    )
}
