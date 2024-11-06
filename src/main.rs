mod components;

use crate::components::layout::Layout;
use leptos::*;

fn main() {
    mount_to_body(|| view! { <Layout/> })
}