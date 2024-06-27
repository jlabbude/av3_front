use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/static/map.js")]
extern "C" {
    fn create_map(div_id: &str);
}

#[function_component(App)]
fn app() -> Html {
    use_effect(|| {
        create_map("map");
        || ()
    });

    html! {
        <div id="map" style="width: 100%; height: 100vh;"></div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
