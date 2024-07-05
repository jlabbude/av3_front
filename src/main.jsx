import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Home from "./pages/Home/Home";
import MapboxExample from "./pages/map/Map";

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/map" element={<MapboxExample />}></Route>
        <Route path="/" element={<Home />}>
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);