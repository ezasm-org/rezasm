import {HashRouter, Route, Routes} from "react-router-dom";
import Code from "./components/Code.jsx";
import Home from "./components/Home.jsx";
import Downloads from "./components/Downloads.jsx";
import "../dist/output.css";

const HOME_PATH = "/";
const CODE_PATH = "/code/";
const DOWNLOAD_PATH = "/downloads/";


/*
    The app itself is routed through the Home page, and through to the code which controls the various
    ide like functions to navigate the code you write, the download path has not been fully worked out, but
    the idea behind it is to work on adding the release versions of rezasm so that the user can choose what to use.

    The code path leads to the component that possesses the code that initializes the Controls, the Console, and the Memory 
    Viewer. It also handles the style css so that the gui can be loaded in. 
*/

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
