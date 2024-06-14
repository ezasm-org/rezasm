import React from "react";
import {STATE} from "./simulator.ts";
import _ from "lodash";

const debounce =
    _.debounce((func) => func(), 250, {leading: true, trailing: false, maxWait: 250});

function Controls({state, setState, start, stop, step, stepBack, reset, load, error}) {
    const isErrorState = error.current !== "";

    return (
        <div className="mt-2 mb-2 row">
            {state.current === STATE.RUNNING ?
                <button className="btn-operation bg-red-500 hover:bg-red-700"
                    disabled={state.current !== STATE.RUNNING || isErrorState}
                    onClick={() => {
                        debounce(stop);
                    }}>
                    Stop
                </button>
                :
                <button className="btn-operation bg-green-500 hover:bg-green-700"
                    disabled={(state.current !== STATE.IDLE && state.current !== STATE.STOPPED) || isErrorState}
                    onClick={() => {
                        debounce(() => {
                            reset()
                                .then(() => load())
                                .then(() => start());
                        });
                    }}>
                    Start
                </button>
            }

            {state.current === STATE.PAUSED ?
                <button className="btn-operation bg-emerald-600 hover:bg-emerald-700"
                    onClick={() => {
                        setState(STATE.RUNNING);
                        debounce(start);
                    }}>
                    Resume
                </button>
                :
                <button className="btn-operation bg-cyan-600 hover:bg-cyan-700"
                    disabled={state.current !== STATE.RUNNING}
                    onClick={() => {
                        setState(STATE.PAUSED);
                    }}>
                    Pause
                </button>
            }

            <button className="btn-operation bg-blue-500 hover:bg-blue-700"
                disabled={(state.current !== STATE.PAUSED && state.current !== STATE.IDLE) || isErrorState}
                onClick={() => {
                    debounce(step);
                }}>
                Step
            </button>
            <button className="btn-operation bg-teal-600 hover:bg-teal-700"
                disabled={state.current !== STATE.PAUSED}
                onClick={() => {
                    debounce(stepBack)
                }}>
                Step Back
            </button>
            <button className="btn-operation bg-orange-500 hover:bg-orange-700"
                onClick={() => {
                    debounce(reset);
                }}>
                Reset
            </button>
        </div>
    );
}

export default Controls;
