import React from "react";
import {useNavigate} from "react-router-dom";
import {CODE_PATH} from "../App.tsx";

/* 
    this is the componenet of the tauri app that handles the initial landing page of the web-app
    This allows you to navigate to the actual code editor, the code path accesses the actual app.
*/

function Home() {

    const navigate = useNavigate();

    return (
        <div>
            <p>Home</p>
            <button onClick={() => navigate(CODE_PATH)}>Take me to the code</button>
        </div>
    );
}

export default Home;
