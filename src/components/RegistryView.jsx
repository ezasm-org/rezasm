import React, {useEffect, useState} from "react";
import {rust_get_register_names, rust_get_register_values} from "../rust_functions.js";
import {CALLBACK_TYPES, CALLBACKS_TRIGGERS} from "../App.jsx";

function RegistryView({loaded, registerCallback}) {
    const [registers, setRegisters] = useState([]);
    const [registerNames, setRegisterNames] = useState([]);

    useEffect(() => {
        if (loaded) {
            rust_get_register_names().then(result => setRegisterNames(result));
            rust_get_register_values().then(result => setRegisters(result));
        }
    }, [loaded]);

    registerCallback(CALLBACKS_TRIGGERS.STEP, CALLBACK_TYPES.REGISTRY,
        () => rust_get_register_values().then(values => setRegisters(values))
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
