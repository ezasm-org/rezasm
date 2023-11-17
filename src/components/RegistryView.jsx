import React, {useEffect, useState} from "react";
import {RUST} from "../rust_functions.js";
import {CALLBACK_TYPES, CALLBACKS_TRIGGERS} from "../App.jsx";

function RegistryView({loaded, registerCallback}) {
    const [registers, setRegisters] = useState([]);
    const [registerNames, setRegisterNames] = useState([]);

    useEffect(() => {
        if (loaded) {
            RUST.GET_REGISTER_NAMES({}).then(result => setRegisterNames(result));
            RUST.GET_REGISTER_VALUES({}).then(result => setRegisters(result));
        }
    }, [loaded]);

    registerCallback(CALLBACKS_TRIGGERS.STEP, CALLBACK_TYPES.REGISTRY,
        () => RUST.GET_REGISTER_VALUES({}).then(values => setRegisters(values))
    );

    let tableData = [];
    for (let i = 0; i < registers.length; ++i) {
        tableData.push(
            <tr>
                <td>{registerNames[i]}</td>
                <td>{Number(registers[i])}</td>
            </tr>
        );
    }

    return (
        <div className="scrollbox">
            <table>
                <thead>
                    <tr>
                        <th>Register</th>
                        <th>Data</th>
                    </tr>
                </thead>
                <tbody>
                    { tableData }
                </tbody>
            </table>
        </div>
    );
}

export default RegistryView;
