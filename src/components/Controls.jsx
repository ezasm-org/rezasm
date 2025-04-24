import React from "react";
import {STATE} from "./simulator.ts";
import _ from "lodash";

// Debounce function to limit the rate at which a function is executed
const debounce =
    _.debounce((func) => func(), 250, {leading: true, trailing: false, maxWait: 250});

/**
 * This component creates the GUI buttons for each control function and also manages the state transitions
 * such as idle, stop, running, etc.
 */
function Controls({state, setState, start, stop, step, stepBack, reset, load, error}) {
    // Check if the current state is an error state
    const isErrorState = error.current !== "";

    return (
        <div className="mt-2 mb-2 row">
            {/* Start/Stop Button */}
            {state.current === STATE.RUNNING ?
                <button className="btn-operation bg-red-500 hover:bg-red-700"
                    disabled={state.current !== STATE.RUNNING || isErrorState}
                    onClick={() => {
                        debounce(stop); // Stop the simulation
                    }}>
                    Stop
                </button>
                :
                <button className="btn-operation bg-green-500 hover:bg-green-700"
                    disabled={(state.current !== STATE.IDLE && state.current !== STATE.STOPPED) || isErrorState}
                    onClick={() => {
                        debounce(() => {
                            reset() // Reset the simulation
                                .then(() => load()) // Load the initial state
                                .then(() => start()); // Start the simulation
                        });
                    }}>
                    Start
                </button>
            }

            {/* Pause/Resume Button */}
            {state.current === STATE.PAUSED ?
                <button className="btn-operation bg-emerald-600 hover:bg-emerald-700"
                    onClick={() => {
                        setState(STATE.RUNNING); // Change state to running
                        debounce(start); // Resume the simulation
                    }}>
                    Resume
                </button>
                :
                <button className="btn-operation bg-cyan-600 hover:bg-cyan-700"
                    disabled={state.current !== STATE.RUNNING}
                    onClick={() => {
                        setState(STATE.PAUSED); // Pause the simulation
                    }}>
                    Pause
                </button>
            }

            {/* Step Button */}
            <button className="btn-operation bg-blue-500 hover:bg-blue-700"
                disabled={(state.current !== STATE.PAUSED && state.current !== STATE.IDLE) || isErrorState}
                onClick={() => {
                    debounce(step); // Execute a single step in the simulation
                }}>
                Step
            </button>

            {/* Step Back Button */}
            <button className="btn-operation bg-teal-600 hover:bg-teal-700"
                disabled={state.current !== STATE.PAUSED}
                onClick={() => {
                    debounce(stepBack); // Go back one step in the simulation
                }}>
                Step Back
            </button>

            {/* Reset Button */}
            <button className="btn-operation bg-orange-500 hover:bg-orange-700"
                onClick={() => {
                    debounce(reset); // Reset the simulation to its initial state
                }}>
                Reset
            </button>
        </div>
    );
}

export default Controls;
