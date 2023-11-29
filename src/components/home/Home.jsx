import React from "react";
import {useNavigate} from "react-router-dom";
import {CODE_PATH} from "../../App.jsx";

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
