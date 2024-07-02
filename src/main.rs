use futures::channel::oneshot;
use geojson::{FeatureCollection, GeoJson};
use log::*;
use mapboxgl::{
    event, LngLat, Map, MapEventListener, MapOptions, Marker, MarkerEventListener, MarkerOptions,
};
use reqwest::Error;
use serde::Deserialize;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Document, Element};
use yew::prelude::*;
use yew::{use_effect_with_deps, use_mut_ref};

struct MarkerListener {}

struct Listener {
    tx: Option<oneshot::Sender<()>>,
}

#[derive(Deserialize, Debug)]
struct Components {
    co: f32,
    no: f32,
    no2: f32,
    o3: f32,
    so2: f32,
    pm2_5: f32,
    pm10: f32,
    nh3: f32,
}

#[derive(Deserialize, Debug)]
struct ListItem {
    components: Components,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    list: Vec<ListItem>,
}

impl MapEventListener for Listener {
    fn on_click(&mut self, _map: Rc<Map>, e: event::MapMouseEvent) {
        let lng_lat = e.lng_lat;
        info!(
            "click: {:?}, {:?}",
            lng_lat.lng.round(),
            lng_lat.lat.round()
        ); // DANGER!! IF COORD VALUES ON MAPPER.JAVA GOES INTO DECIMAL VALUES, THIS'LL BREAK

        let url = format!(
            "http://localhost:8080/data?x={}&y={}",
            lng_lat.lng.round(),
            lng_lat.lat.round()
        );

        spawn_local(async move {
            let response = reqwest::get(&url).await.unwrap();
            let _data = response.text().await.unwrap();
        });
    }

    fn on_load(&mut self, m: Rc<Map>, _e: event::MapBaseEvent) {
        self.tx.take().unwrap().send(()).unwrap();

        let map2 = m.clone();
        m.load_image("http://localhost:8080/pollution", move |res| {
            if let Ok(image) = res {
                info!("image loaded: {:?}", &image.inner);

                map2.add_image("pollution", image, mapboxgl::ImageOptions::default())
                    .unwrap();
                info!("image added");

                map2.add_geojson_source(
                    "pollutionpol",
                    GeoJson::FeatureCollection(FeatureCollection {
                        bbox: None,
                        foreign_members: None,
                        features: vec![geojson::Feature {
                            bbox: None,
                            geometry: Some(geojson::Geometry::new(
                                geojson::Value::Polygon(vec![vec![
                                    vec![-180.0, -90.0],
                                    vec![180.0, -90.0],
                                    vec![180.0, 90.0],
                                    vec![-180.0, 90.0],
                                    vec![-180.0, -90.0],
                                ]]),
                            )),
                            id: None,
                            properties: None,
                            foreign_members: None,
                        }],
                    }),
                )
                    .unwrap();
                info!("source added");

                map2.add_layer(&mapboxgl::Layer {
                    id: "polid".into(),
                    r#type: "fill".into(),
                    source: "pollutionpol".into(),
                    paint: None,
                    layout: None,
                })
                    .unwrap();
                info!("layer added");
            }
        });
    }
}

impl MarkerListener {
    async fn fetch_components(m: Rc<Marker>) -> Result<ApiResponse, Error> {
        let lnglat = m.get_lnglat();
        let url = format!(
            "http://localhost:8080/data?x={}&y={}",
            lnglat.lng().round(),
            lnglat.lat().round()
        );
        info!("fetching components from: {}", url);
        let response = reqwest::get(&url).await?;
        let json = response.json::<ApiResponse>().await?;
        return Ok(json);
    }
}

impl MarkerEventListener for MarkerListener {
    fn on_dragend(&mut self, m: Rc<Marker>, _e: event::DragEvent) {
        let document: Document = web_sys::window().unwrap().document().unwrap();
        let values: Element = document.get_element_by_id("values").unwrap();
        values.set_attribute("style", "display: block;").unwrap();
        spawn_local(async move {
            if let Ok(api) = MarkerListener::fetch_components(m).await {
                let components = &api.list[0].components;

                values.set_inner_html(&format!(
                    "co {}<br/>no {}<br/>no2 {}<br/>o3 {}<br/>so2 {}<br/>pm2_5 {}<br/>pm10 {}<br/>nh3 {}",
                    components.co, components.no, components.no2, components.o3, components.so2, components.pm2_5, components.pm10, components.nh3,
                ));

                info!("dragend: {:?}", components);
            } else {
                info!("Failed to fetch components");
            }
        });
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

                // add marker
                let mut marker_options = MarkerOptions::new();
                marker_options.draggable = Some(true);
                let marker =
                    Marker::with_listener(LngLat::new(0.0, 0.0), marker_options, MarkerListener {});
                m.add_marker(marker);

                spawn_local(async move {
                    rx.await.unwrap();
                    if let Ok(mut map) = map.try_borrow_mut() {
                        info!("map loaded");
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
        <pre id="values" class="values"></pre>
      </div>
    }
}

pub fn create_map() -> Rc<Map> {
    let token = "pk.eyJ1IjoiZW56b2Zlcm5hbmRlczEyMyIsImEiOiJjbHh5M25nMmoyemJ1MnJwenJyaGdtczhvIn0.0BuZylH0eB-djtYNwyjW6w";

    let opts = MapOptions::new(token.into(), "map".into())
        .center(LngLat::new(0.0, 0.0))
        //.style("mapbox://styles/mapbox/streets-v12".into())
        .zoom(3.0);

    Map::new(opts).unwrap()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
