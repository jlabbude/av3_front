import Map from '/home/lucas/Codigo/Rust/av3_front/node_modules/ol/Map';
import OSM from '/home/lucas/Codigo/Rust/av3_front/node_modules/ol/source/OSM'
import TileLayer from '/home/lucas/Codigo/Rust/av3_front/node_modules/ol/layer/Tile';
import View from '/home/lucas/Codigo/Rust/av3_front/node_modules/ol/View';

export function create_map(div_id) {
    const map = new Map({
        target: div_id,
        layers: [
            new TileLayer({
                source: new OSM(),
            }),
        ],
        view: new View({
            center: [0, 0],
            zoom: 2,
        }),
    });
}
