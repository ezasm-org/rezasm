import React from "react";
import {HashRouter, Route, Routes} from "react-router-dom";
import Code from "./components/Code.jsx";
import Home from "./components/Home.jsx";
import Downloads from "./components/Downloads.jsx";
import "../dist/output.css";

const HOME_PATH = "/";
const CODE_PATH = "/code/";
const DOWNLOAD_PATH = "/downloads/";

function App() {
    return (
        <HashRouter future={{ v7_startTransition: true }}>
            <Routes>
                <Route path={HOME_PATH} element={<Home />} />
                <Route path={CODE_PATH} element={<Code />} />
                <Route path={DOWNLOAD_PATH} element={<Downloads />} />
            </Routes>
        </HashRouter>
    );
}

export default App;

export { HOME_PATH, CODE_PATH, DOWNLOAD_PATH };
