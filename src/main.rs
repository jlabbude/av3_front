use futures::channel::oneshot;
use log::*;
use mapboxgl::{event, LngLat, Map, MapEventListener, MapOptions, QueryFeatureOptions};
use serde::Serialize;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref};

struct Listener {
    tx: Option<oneshot::Sender<()>>,
}

impl MapEventListener for Listener {

    fn on_click(&mut self, map: Rc<Map>, e: event::MapMouseEvent) {
        let lng_lat = e.lng_lat;

        let url = format!(
            "http://localhost:8080/data?lng={}&lat={}",
            lng_lat.lng, lng_lat.lat
        );

        wasm_bindgen_futures::spawn_local(async move {
            let response = reqwest::get(&url).await.unwrap();
            let data = response.text().await.unwrap();

            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("features")
                .expect("Element \"features\" not found")
                .set_inner_html(&data);
        });
    }

    fn on_load(&mut self, _map: Rc<Map>, _e: event::MapBaseEvent) {
        self.tx.take().unwrap().send(()).unwrap();
    }


}

#[hook]
fn use_map() -> Rc<RefCell<Option<Rc<Map>>>> {
    let map = use_mut_ref(|| Option::<Rc<Map>>::None);

    {
        let map = map.clone();
        use_effect_with_deps(
            move |_| {
                let m = create_map();

                let (tx, rx) = oneshot::channel();
                let _ = m.on(Listener { tx: Some(tx) }).unwrap();

                wasm_bindgen_futures::spawn_local(async move {
                    rx.await.unwrap();
                    info!("map loaded");
                    if let Ok(mut map) = map.try_borrow_mut() {
                        map.replace(m);
                    } else {
                        error!("Failed to create Map");
                    }
                });
                || {}
            },
            (),
        );
    }

    map
}

#[function_component(App)]
fn app() -> Html {
    let _map = use_map();

    html! {
      <div>
        <div id="map" style="width: 100vw; height: 100vh;"></div>
        <pre id="features"></pre>
      </div>
    }
}

pub fn create_map() -> Rc<Map> {
    let token = "pk.eyJ1IjoiZW56b2Zlcm5hbmRlczEyMyIsImEiOiJjbHh5M25nMmoyemJ1MnJwenJyaGdtczhvIn0.0BuZylH0eB-djtYNwyjW6w";

    let opts = MapOptions::new(token.into(), "map".into())
        .center(LngLat::new(0.0,0.0))
        //.style("mapbox://styles/mapbox/streets-v12".into())
        .zoom(3.0);

    Map::new(opts).unwrap()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}