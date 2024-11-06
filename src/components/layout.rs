use leptos::*;
use crate::components::markdown::Mdx;

#[component]
pub fn Layout() -> impl IntoView {

    let markdown_content = String::from(include_str!("../../blog/Meows.md"));

    view! {
        <div>
            <p class="underline text-red-900">"hello world"</p>
            <Mdx markdown=markdown_content />
        </div>
    }
}
