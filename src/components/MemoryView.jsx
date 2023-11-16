import {rust_get_memory_bounds, rust_get_memory_slice, rust_get_word_size} from "../rust_functions.js";
import React, {useCallback, useEffect, useRef, useState} from "react";
import {CALLBACK_TYPES, CALLBACKS_TRIGGERS} from "../App.jsx";

const WIDTH = 4;
const HEIGHT = 4;
const CELLS = WIDTH * HEIGHT;

function MemoryView({loaded, registerCallback}) {
    let lowest = useRef(0);
    let text = useRef(0);
    let heap = useRef(0);
    let stack = useRef(0);
    let [currentAddress, setCurrentAddress] = useState(0);
    let [slice, setSlice] = useState(Array(CELLS).fill(0));
    let wordSize = useRef(4);

    let [addressInput, setAddressInput] = useState("0x" + currentAddress.toString(16));
    let [selectorOptions, setSelectorOptions] = useState([0, 0, 0]);
    let [selected, setSelected] = useState(0);

    const updateSlice = useCallback(async address => {
        if (address >= lowest.current && address <= (stack.current - CELLS * wordSize.current)) {
            setCurrentAddress(address);
            setAddressInput("0x" + address.toString(16));
            let array = await rust_get_memory_slice(address, CELLS);
            let numberArray = [];
            for (let i = 0; i < array.length; ++i) {
                numberArray.push(Number(array[i]));
            }
            setSlice(numberArray);
        }
    }, []);

    const updateSliceCurrent = useCallback(async () => {
        updateSlice(currentAddress);
    }, [currentAddress, updateSlice]);

    registerCallback(CALLBACKS_TRIGGERS.STEP, CALLBACK_TYPES.MEMORY, updateSliceCurrent);

    useEffect(() => {
        if (loaded) {
            rust_get_word_size().then(rustWordSize => {
                wordSize.current = rustWordSize;
                rust_get_memory_bounds().then(bounds => {
                    lowest.current = Number(bounds[0]);
                    text.current = Number(bounds[0]);
                    heap.current = Number(bounds[1]);
                    stack.current = Number(bounds[2]);
                    let address = stack.current - CELLS * wordSize.current;
                    updateSlice(address);
                    setSelectorOptions([address, heap.current, text.current]);
                    setSelected(stack.current);
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

    const seek = useCallback(() => {
        let address = parseInt(addressInput);
        if (isNaN(address) || address < lowest.current) {
            setAddressInput("0x" + currentAddress.toString(16));
        } else {
            updateSlice(address);
        }
    }, [addressInput, currentAddress, updateSlice]);

    const seekLeft = useCallback(() => {
        let address = currentAddress - (CELLS * wordSize.current);
        if (!isNaN(address)) {
            updateSlice(address);
        }
    }, [currentAddress, updateSlice]);

    const seekRight = useCallback(() => {
        let address = currentAddress + (CELLS * wordSize.current);
        if (!isNaN(address)) {
            updateSlice(address);
        }
    }, [currentAddress, updateSlice]);

    return (
        <div className="scrollbox">
            <div className="control-panel row fill">
                <select value={selected} onChange={(e) => {
                    let selection = Number(e.target.value);
                    setSelected(selection);
                    updateSlice(selection);
                }}>
                    <option value={selectorOptions[0]}>Stack Top</option>
                    <option value={selectorOptions[1]}>Heap Bottom</option>
                    <option value={selectorOptions[2]}>Text Section</option>
                </select>
                <input
                    className="address-input"
                    value={addressInput}
                    onChange={(e) => {
                        setAddressInput(e.target.value);
                    }}
                />
                <button className="btn-basic" onClick={seek}>Seek</button>
                <button className="btn-basic" onClick={seekLeft}>&lt;--</button>
                <button className="btn-basic" onClick={seekRight}>--&gt;</button>
            </div>
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
