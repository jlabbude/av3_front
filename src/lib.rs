use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

#[wasm_bindgen(module = "ol/Map")]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object)]
    type Map;

    #[wasm_bindgen(constructor)]
    fn new(options: &JsValue) -> Map;
}

#[wasm_bindgen(module = "ol/View")]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object)]
    type View;

    #[wasm_bindgen(constructor)]
    fn new(options: &JsValue) -> View;
}

#[wasm_bindgen(module = "ol/layer/Tile")]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object)]
    type TileLayer;

    #[wasm_bindgen(constructor)]
    fn new(options: &JsValue) -> TileLayer;
}

#[wasm_bindgen(module = "ol/source/OSM")]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object)]
    type OSM;

    #[wasm_bindgen(constructor)]
    fn new() -> OSM;
}

#[wasm_bindgen]
pub fn create_map(div_id: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let map_div = document.get_element_by_id(div_id).unwrap();

    let map_div: HtmlElement = map_div.dyn_into::<HtmlElement>().unwrap();

    let osm_source = OSM::new();
    let tile_layer_options = js_sys::Object::new();
    js_sys::Reflect::set(&tile_layer_options, &JsValue::from_str("source"), &osm_source).unwrap();
    let tile_layer = TileLayer::new(&tile_layer_options);

    let view_options = js_sys::Object::new();
    js_sys::Reflect::set(&view_options, &JsValue::from_str("center"), &JsValue::from_serde(&[0.0, 0.0]).unwrap()).unwrap();
    js_sys::Reflect::set(&view_options, &JsValue::from_str("zoom"), &JsValue::from_f64(2.0)).unwrap();
    let view = View::new(&view_options);

    let map_options = js_sys::Object::new();
    js_sys::Reflect::set(&map_options, &JsValue::from_str("target"), &JsValue::from(map_div)).unwrap();
    js_sys::Reflect::set(&map_options, &JsValue::from_str("layers"), &js_sys::Array::of1(&tile_layer)).unwrap();
    js_sys::Reflect::set(&map_options, &JsValue::from_str("view"), &view).unwrap();

    let _map = Map::new(&map_options);
}
