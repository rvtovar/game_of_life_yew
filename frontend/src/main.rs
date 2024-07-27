mod life_lib;
mod universe_component;

use yew::prelude::*;
use universe_component::UniverseComponent;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <UniverseComponent />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}