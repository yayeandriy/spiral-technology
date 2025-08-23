use leptos::{prelude::*, component};

/// Convert a markdown string into safe HTML and render it.
///
/// *If you’re sure your source is trusted, you can skip `ammonia`.
/// But in any public‑facing app you should sanitize the output!*
#[component]
pub fn MarkdownRenderer(text: String) -> impl IntoView {
    // Parse the markdown once and cache it.
    // We use a `Signal` so the component will re‑render if the text changes.
    let html = move || {
        // 1️⃣ Parse the markdown
        let markdown_input = text;
        let options = pulldown_cmark::Options::all();
        let parser = pulldown_cmark::Parser::new_ext(&markdown_input, options);

        // 2️⃣ Convert parser to a string of raw HTML
        let mut html_raw = String::new();
        pulldown_cmark::html::push_html(&mut html_raw, parser);

        // 3️⃣ (Optional but recommended) Sanitize the HTML
        ammonia::clean(&html_raw)
    };

    view! {
        // `dangerously_set_inner_html` takes a `String` and inserts it as raw
        // inner‑HTML.  It is *dangerous* only if the markup is not sanitized.
        <div class="prose w-full" inner_html={html()} />
    }
}
