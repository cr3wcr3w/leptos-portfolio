use leptos::*;
use rand::random;

// use crate::components::markdown::Mdx;

#[component]
pub fn Layout() -> impl IntoView {

    // let markdown_content = String::from(include_str!("../../blog/Meows.md"));

    fn generate_start(n: u32) -> String {
        let mut value = format!(
            "box-shadow:{}px {}px rgba(165, 190, 195, {})",
            (random::<f64>() * 2000.0).floor() as u32,
            (random::<f64>() * 2000.0).floor() as u32,
            (random::<f64>() * 1.0).to_string()
        );

        for _i in 0..n {
            value += &format!(
                ", {}px {}px rgba(184, 174, 131, {})",
                (random::<f64>() * 2000.0).floor() as u32,
                (random::<f64>() * 2000.0).floor() as u32,
                (random::<f64>() * 1.0).to_string()
            );
        }
        value
    }

    view! {
        <>
            // <Mdx markdown=markdown_content />

            // stars
            <div id="stars1" style={generate_start(200)}></div>
            <div id="stars2" style={generate_start(300)}></div>
            <div id="stars3" style={generate_start(400)}></div>
            <div id="stars4" style={generate_start(500)}></div>
            
            <header>header</header>
            <main>main</main>
            <footer>footer</footer>
            
        </>
    }
}
