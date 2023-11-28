import {useCallback, useReducer, useRef, useState} from "react";
import {RUST} from "../../rust_functions.js";

const STATE = {
    IDLE: 1,
    LOADING: 2,
    LOADED: 3,
    RUNNING: 4,
    PAUSED: 5,
    STOPPED: 6,
};

const CALLBACKS_TRIGGERS = {
    RESET: "RESET",
    STEP: "STEP"
};

const CALLBACK_TYPES = {
    CONSOLE: "CONSOLE",
    MEMORY: "MEMORY",
    REGISTRY: "REGISTRY",
};

let initialCallbacks = {};
Object.values(CALLBACKS_TRIGGERS).map(x => initialCallbacks[x] = {});

export const useSimulator = () => {
    const state = useRef(STATE.IDLE);
    const [code, setCode] = useState("");
    const [error, setError] = useState("");
    const [exitCode, setExitCode] = useState("");

    const timerId = useRef(null);
    const [instructionDelay, setInstructionDelay] = useState(5);
    const callbacks = useRef(initialCallbacks);

    //Still kind of a hack
    const [, forceUpdate] = useReducer(() => Date.now());

    const setState = (newState) => {
        state.current = newState;
        forceUpdate();
    };

    const registerCallback = (trigger, type, callback) => {
        callbacks.current[trigger][type] = callback;
    };

    const callStepCallbacks = () => {
        Object.values(callbacks.current[CALLBACKS_TRIGGERS.STEP]).map(callback => callback());
    };

    const callResetCallbacks = () => {
        Object.values(callbacks.current[CALLBACKS_TRIGGERS.RESET]).map(callback => callback());
    };

    const isError = useCallback(() => {
        return error !== "";
    }, [error]);

    const haltExecution = useCallback((newState) => {
        setState(newState ?? STATE.STOPPED);
        if (timerId.current !== null) {
            clearTimeout(timerId.current);
            timerId.current = null;
        }
    }, []);

    const stop = useCallback(async () => {
        haltExecution(STATE.STOPPED);
        await RUST.STOP({});
    }, []);

    const reset = useCallback(async () => {
        haltExecution(STATE.IDLE);
        await RUST.RESET({});
        callStepCallbacks();
        callResetCallbacks();
        setExitCode("");
        setError("");
    }, []);

    const load = useCallback(async () => {
        if (state.current < STATE.LOADED) {
            setState(STATE.LOADING);
            return RUST.LOAD({lines: code})
                .then(() => {
                    setState(STATE.LOADED);
                })
                .catch(error => {
                    setError(error);
                    setState(STATE.STOPPED);
                });
        }
    }, [code]);

    const checkProgramCompletion = useCallback(async () => {
        if (await RUST.IS_COMPLETED({}) || isError()) {
            haltExecution(STATE.STOPPED);
            setExitCode("" + await RUST.GET_EXIT_STATUS({}));
            return true;
        } else {
            return false;
        }
    }, [haltExecution, isError]);

    const handleStepCall = useCallback(async () => {
        RUST.STEP({})
            .then(async () => {
                await checkProgramCompletion();
                callStepCallbacks();
            })
            .catch(error => {
                setError(error);
                setState(STATE.STOPPED); // maybe add STATE.ERROR?
            });
    }, [checkProgramCompletion]);

    const step = useCallback(async () => {
        if (state.current < STATE.LOADED) {
            await reset();
            await load();
            if (state.current !== STATE.STOPPED && ! await checkProgramCompletion()) {
                return handleStepCall().then(() => setState(STATE.PAUSED));
            }
        } else if (state.current === STATE.PAUSED || state.current === STATE.RUNNING) {
            return handleStepCall();
        }
    }, [checkProgramCompletion, handleStepCall, load, reset, state]);

    const recursiveStep = useCallback(async () => {
        if (state.current === STATE.STOPPED) {
            return;
        }
        checkProgramCompletion().then(async completed => {
            if (!completed && state.current === STATE.RUNNING) {
                handleStepCall().then(() => {
                    timerId.current = setTimeout(recursiveStep, instructionDelay);
                }).catch((e) => {
                    timerId.current = null;
                    setError(e);
                    setState(STATE.STOPPED);
                });
            }
        });
    }, [checkProgramCompletion, handleStepCall, instructionDelay, state]);

    const start = useCallback(() => {
        setState(STATE.RUNNING);
        recursiveStep();
    });

    return {
        state,
        error,
        exitCode,
        setState,
        setCode,
        setInstructionDelay,
        registerCallback,
        start,
        stop,
        step,
        load,
        reset,
    };
};

export {STATE, CALLBACKS_TRIGGERS, CALLBACK_TYPES};
