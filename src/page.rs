use crate::{html, view::View};

pub fn page(component: View, title: &str) -> View {
    page_inner(component, View::default(), title)
}

pub fn page_with_view_before(component: View, before: View, title: &str) -> View {
    page_inner(component, before, title)
}

fn page_inner(component: View, before: View, title: &str) -> View {
    let style_url = "/assets/styles.css?version=14";
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
            </head>
            <body
                class="text-black bg-white dark:text-white dark:bg-neutral-900"
                hx-history="false"
                hx-boost="true"
            >

                <div class="sticky top-0 flex bg-inherit flex-col">
                    <nav class="w-full text-xl px-3 py-2 bg-slate-800 flex gap-3 items-center">
                        <a href="https://www.festorkestret.dk/">
                            <img class="w-60 p-2" src="https://www.festorkestret.dk/wp-content/uploads/2018/06/FO-font-white.png" />
                        </a>
                        Setliste
                    </nav>
                    {before}
                </div>


                <div class="flex w-full justify-center p-4">
                    {component}
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
