use leptos::*;
use pulldown_cmark::{Parser, html};

#[component]
pub fn Mdx(markdown: String) -> impl IntoView {

    fn parse_markdown(markdown: &str) -> String {
        let parser = Parser::new(markdown);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }

    // Parse the Markdown into HTML
    let html_output = parse_markdown(&markdown);

    // Return the view with the parsed HTML
    view! {
        <div class="markdown-body" inner_html=html_output></div>
    }
}