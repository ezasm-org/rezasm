import {rust_get_memory_bounds, rust_get_memory_slice, rust_get_word_size} from "../rust_functions.js";
import React, {useCallback, useEffect, useRef, useState} from "react";
import {CALLBACKS} from "../App.jsx";

const WIDTH = 4;
const HEIGHT = 5;
const CELLS = WIDTH * HEIGHT;

function MemoryView({loaded, registerCallback}) {
    let lowest = useRef(0);
    let text = useRef(0);
    let heap = useRef(0);
    let stack = useRef(0);
    let [currentAddress, setCurrentAddress] = useState(0);
    let [slice, setSlice] = useState(Array(CELLS).fill(0));
    let wordSize = useRef(4);

    const updateSlice = useCallback(async address => {
        if (address >= lowest.current && address <= (stack.current - CELLS * wordSize.current)) {
            setCurrentAddress(address);
            let array = await rust_get_memory_slice(address, CELLS);
            let numberArray = [];
            for (let i = 0; i < array.length; ++i) {
                numberArray.push(Number(array[i]));
            }
            setSlice(numberArray);
        }
    }, [lowest, stack]);

    const updateSliceCurrent = useCallback(async () => {
        updateSlice(currentAddress);
    }, [currentAddress, updateSlice]);

    registerCallback(CALLBACKS.MEMORY, updateSliceCurrent);

    useEffect(() => {
        if (loaded) {
            rust_get_word_size().then(rustWordSize => {
                wordSize.current = rustWordSize;
                rust_get_memory_bounds().then(bounds => {
                    text.current = Number(bounds[0]);
                    heap.current = Number(bounds[1]);
                    stack.current = Number(bounds[2]);
                    let address = stack.current - CELLS * wordSize.current;
                    updateSlice(address);
                });
            });
        }
    }, [loaded, updateSlice]);

    let count = 0;

    let headers = Array(WIDTH).fill(0).map(() => `+0x${(count++ * wordSize.current).toString(16)}`);

    let table = [];

    while (table.length < slice.length / WIDTH) {
        table.push(slice.slice(table.length * WIDTH, (table.length + 1) * WIDTH));
    }

    count = 0;

    return (
        <div className="scrollbox">
            <table className="fill">
                <thead>
                    <tr>
                        <td></td>
                        { headers.map(value => <td key={value}>{value}</td>) }
                    </tr>
                </thead>
                <tbody>
                    {table.map(row =>
                        <tr key={count}>
                            <td>{`0x${(currentAddress + count++ * wordSize.current * WIDTH).toString(16)}`}</td>
                            {row.map(value =>
                                <td>
                                    {`0x${value.toString(16)}`}
                                </td>
                            )}
                        </tr>
                    )}
                </tbody>
            </table>
        </div>
    );
}

export default MemoryView;
