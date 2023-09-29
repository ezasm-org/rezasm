import React from "react";

function RegisterView({registerNames, registers}) {
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
                    { tableData.map(data => data) }
                </tbody>
            </table>
        </div>
    );
}

export default RegisterView
