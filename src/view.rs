#[macro_export]
macro_rules! html {
    ($($body:tt)*) => {
        View::new(rstml_to_string_macro::html!($($body)*))
    };
}

#[derive(Clone)]
pub struct View(String);

impl View {
    pub fn new(string: String) -> View {
        View(string)
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl axum::response::IntoResponse for View {
    fn into_response(self) -> axum::response::Response {
        (
            [
                (
                    axum::http::header::CONTENT_TYPE,
                    axum::http::HeaderValue::from_static(mime::TEXT_HTML_UTF_8.as_ref()),
                ),
                (
                    axum::http::header::CACHE_CONTROL,
                    axum::http::HeaderValue::from_static("no-cache"),
                ),
            ],
            self.0,
        )
            .into_response()
    }
}

impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromIterator<View> for View {
    fn from_iter<T: IntoIterator<Item = View>>(iter: T) -> Self {
        let mut result = String::new();

        for view in iter {
            result += &view.0;
        }
        View(result)
    }
}
