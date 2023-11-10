import {rust_get_memory_bounds, rust_get_memory_slice, rust_get_word_size} from "../rust_functions.js";
import React, {useCallback, useEffect, useRef, useState} from "react";
import {CALLBACKS} from "../App.jsx";
import { words } from "lodash";

const WIDTH = 4;
const HEIGHT = 4;
const CELLS = WIDTH * HEIGHT;

function MemoryView({loaded, registerCallback}) {
    let lowest = useRef(0); //useRef(0) makes lowest.current = 0 initially
    let text = useRef(0);
    let heap = useRef(0);
    let stack = useRef(0);
    let [currentAddress, setCurrentAddress] = useState(0); //currentAddress is a state variable (state persists through re-renders, use setCurrentAddress to update currentAddress which will cause page to re-render)
    let [slice, setSlice] = useState(Array(CELLS).fill(0)); //Array(CELLS).fill(0) creates an array of size 16 filled with 0s
    let wordSize = useRef(4);

    // stuff about toggle hex/decimal
    let [toggle, setToggle] = useState(["0x", 16]);
    const updateToggle = useCallback(() => {
        if (toggle[1] == 10) {
            setToggle(["0x",16]);
            // toggle[0] = "0x";
            // toggle[1] = 16;
        } else {
            setToggle(["",10]);
            // toggle[0] = "";
            // toggle[1] = 10;
        }
        console.log(toggle[0] + " " + toggle[1]);
    }, [toggle, setToggle]);

    // let [addressInput, setAddressInput] = useState("0x" + currentAddress.toString(16)); //converting currentAddress to a hexadecimal number as a string and appending 0x in front of it
    let [addressInput, setAddressInput] = useState(toggle[0] + currentAddress.toString(toggle[1]));
    let [selectorOptions, setSelectorOptions] = useState([0, 0, 0]);
    let [selected, setSelected] = useState(0);

    const updateSlice = useCallback(async address => {
        if (address >= lowest.current && address <= (stack.current - CELLS * wordSize.current)) { //checking if address is valid?
            setCurrentAddress(address); //update currentAddress to address
            // setAddressInput("0x" + address.toString(16)); //update addressInput to hex version of address
            setAddressInput(toggle[0] + address.toString(toggle[1]));
            let array = await rust_get_memory_slice(address, CELLS); //gives an array of addresses from address to address + cells*4
            let numberArray = [];
            for (let i = 0; i < array.length; ++i) { //converting array[i] into numbers
                numberArray.push(Number(array[i]));
            }
            setSlice(numberArray); //update slice to numberArray
        }
    }, []);

    const updateSliceCurrent = useCallback(async () => {
        updateSlice(currentAddress);
    }, [currentAddress, updateSlice]); //currentAddress and updateSlice are dependencies because if they change, then they need to know to be updated?

    registerCallback(CALLBACKS.MEMORY, updateSliceCurrent); // callbacks = {..., MEMORY => updateSliceCurrent()}

    useEffect(() => { //useEffect is used here to make React work with stuff from rust
        if (loaded) { //wasmLoaded = true
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

    // let headers = Array(WIDTH).fill(0).map(() => `+0x${(count++ * wordSize.current).toString(16)}`); //sets headers of memory viewer to +0x0, +0x4, +0x8, +0xc
    let headers = Array(WIDTH).fill(0).map(() => `+${toggle[0]}${(count++ * wordSize.current).toString(toggle[1])}`);

    let table = [];

    while (table.length < slice.length / WIDTH) { //table.length < 4
        table.push(slice.slice(table.length * WIDTH, (table.length + 1) * WIDTH)); //[0-4], [4-8], [8-12], [12-16]
    } //this seems to pretty much just copy all of slice to table

    count = 0;

    const seek = useCallback(() => { //goes to address that user inputted
        let address = parseInt(addressInput); //address that user inputted
        if (isNaN(address) || address < lowest.current) { //if address isn't valid, then go to currentAddress
            // setAddressInput("0x" + currentAddress.toString(16));
            setAddressInput(toggle[0] + currentAddress.toString(toggle[1]));
        } else {
            updateSlice(address); //updates currentAddress to addressInput
        }
    }, [addressInput, currentAddress, updateSlice]);

    const seekLeft = useCallback(() => { //goes to previous 4 addresses
        let address = currentAddress - (CELLS * wordSize.current);
        if (!isNaN(address)) {
            updateSlice(address);
        }
    }, [currentAddress, updateSlice]);

    const seekRight = useCallback(() => { //goes to next 4 addresses
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
                <button className="btn-basic" onClick={seekLeft}>&lt;---</button>
                <button className="btn-basic" onClick={seekRight}>---&gt;</button>
                <button className="btn-basic" onClick={updateToggle}>Toggle Hex/Decimal</button>
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
                            {/* <td>{`0x${(currentAddress + count++ * wordSize.current * WIDTH).toString(16)}`}</td>  */}
                            <td>{`${toggle[0]}${(currentAddress + count++ * wordSize.current * WIDTH).toString(toggle[1])}`}</td>
                            {row.map(value =>
                                <td>
                                    {/* {`0x${value.toString(16)}`} */}
                                    {`${toggle[0]}${value.toString(toggle[1])}`}
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
