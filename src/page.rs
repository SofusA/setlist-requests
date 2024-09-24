use crate::{html, view::View};

pub fn page(component: View, title: &str) -> View {
    let style_url = "/assets/styles.css?version=6";
    let doctype = "<!DOCTYPE html>";

    html! {
        {doctype}

        <html lang="en" class="h-full dark">
            <head>
                <title>{title}</title>
                <link rel="icon" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🎵</text></svg>">
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <link rel="stylesheet" href=style_url />
                <script src="https://unpkg.com/htmx.org@2.0.0"></script>
                <script src="https://cdn.jsdelivr.net/gh/gnat/surreal@main/surreal.js"></script>
                <script src="https://unpkg.com/feather-icons"></script>
            </head>
            <body
                class="text-black bg-white dark:text-white dark:bg-neutral-950"
                hx-history="false"
            >

                <nav class="w-full text-xl px-3 py-2 sticky top-0 bg-neutral-900">FestOrkestret Setlist</nav>

                <div class="flex w-full justify-center p-4">
                    <div class="flex max-w-7xl">{component}</div>
                </div>

                <script src="/assets/scripts/htmx-config.js?version=2"></script>

                {if cfg!(debug_assertions) {
                    html! { <script src="/assets/scripts/develop-updates.js"></script> }
                } else {
                    Default::default()
                }}

            </body>
        </html>
    }
}
