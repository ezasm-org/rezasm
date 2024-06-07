import {useCallback, useReducer, useRef, useState} from "react";
import {RUST} from "../rust_functions.js";

enum STATE {
    IDLE = 1,
    LOADING = 2,
    LOADED = 3,
    RUNNING = 4,
    PAUSED = 5,
    STOPPED = 6,
}

const CALLBACKS_TRIGGERS = {
    RESET: "RESET",
    STEP: "STEP"
} as const;

type ValidCallbackTriggers = keyof typeof CALLBACKS_TRIGGERS;

const CALLBACK_TYPES = {
    CONSOLE: "CONSOLE",
    MEMORY: "MEMORY",
    REGISTRY: "REGISTRY",
} as const;

type ValidCallbackTypes = keyof typeof CALLBACK_TYPES;


type CallbackTriggerObject = Partial<Record<ValidCallbackTypes, () => unknown>>;
type CallbackObject = Record<ValidCallbackTriggers, CallbackTriggerObject>;
const initialCallbacks: CallbackObject = Object.values(CALLBACKS_TRIGGERS).reduce((callbacks,  x) => {
    callbacks[x] = {};
    return callbacks;
}, {} as Partial<CallbackObject>) as CallbackObject;

export const useSimulator = () => {
    const state = useRef(STATE.IDLE);
    const error = useRef("");
    const [exitCode, setExitCode] = useState("");
    const [code, setCode] = useState("");

    const timerId = useRef<number|null>(null);
    const [instructionDelay, setInstructionDelay] = useState(5);
    const callbacks = useRef(initialCallbacks);

    //Still kind of a hack
    const [, forceUpdate] = useReducer(() => Date.now(), 0);

    const setState = useCallback((newState: STATE) => {
        state.current = newState;
        forceUpdate();
    }, []);

    const setError = useCallback((newError: string) => {
        error.current = newError;
        forceUpdate();
    }, []);

    const registerCallback = (trigger: ValidCallbackTriggers, type: ValidCallbackTypes, callback: () => unknown) => {
        callbacks.current[trigger][type] = callback;
    };

    const callStepCallbacks = useCallback(() => {
        Object.values(callbacks.current[CALLBACKS_TRIGGERS.STEP]).map(callback => callback());
    }, [callbacks]);

    const callResetCallbacks = useCallback(() => {
        Object.values(callbacks.current[CALLBACKS_TRIGGERS.RESET]).map(callback => callback());
    }, [callbacks]);

    const haltExecution = useCallback((newState: STATE) => {
        setState(newState ?? STATE.STOPPED);
        if (timerId.current !== null) {
            clearTimeout(timerId.current);
            timerId.current = null;
        }
    }, [timerId]);

    const isError = () => {
        return error.current !== "";
    };

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
                    setState(STATE.STOPPED);
                    setError(error);
                });
        }
    }, [code]);

    const checkProgramCompletion = useCallback(async () => {
        if (isError()) {
            haltExecution(STATE.STOPPED);
            setExitCode("");
            return true;
        } else if (await RUST.IS_COMPLETED({})) {
            haltExecution(STATE.STOPPED);
            setExitCode("" + await RUST.GET_EXIT_STATUS({}));
            return true;
        } else {
            return false;
        }
    }, []);

    const handleStepCall = useCallback(async () => {
        RUST.STEP({})
            .then(async () => {
                await checkProgramCompletion();
                callStepCallbacks();
            })
            .catch((error) => {
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

    const stepBack = useCallback(async () => {
        if (state.current > STATE.RUNNING) {
            console.log(state.current);
            RUST.STEP_BACK({})
                .catch((error) => {
                    setError(error);
                    setState(STATE.STOPPED);
                })
                .finally(() => {
                    callStepCallbacks();
                });

        }
    }
    , [setError, setState, callStepCallbacks]);

    const recursiveStep = useCallback(async () => {
        if (state.current === STATE.STOPPED) {
            return;
        }
        checkProgramCompletion().then(async completed => {
            if (!completed && state.current === STATE.RUNNING) {
                handleStepCall().then(() => {
                    // @ts-expect-error -- It assumes that setTimeout returns a NodeJS.Timeout object,
                    // which does not exist in the browser
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
    }, [recursiveStep]);

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
        stepBack,
        load,
        reset,
    };
};

export {STATE, CALLBACKS_TRIGGERS, CALLBACK_TYPES};
