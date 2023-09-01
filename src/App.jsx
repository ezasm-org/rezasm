import { useState } from "react";
import reactLogo from "./assets/react.svg";
import ezasmLogo from "./assets/ezasm.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [result, setResult] = useState("");
  const [line, setLine] = useState("");

  async function run() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      setResult(await invoke("run", { line }));
  }

  return (
    <div className="container">
      <h1>Welcome to rezasm!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
        <a href="" target="_blank">
          <img src={ezasmLogo} className="logo react" alt="EzASM logo"></img>
        </a>
      </div>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          run();
        }}
      >
        <textarea
          id="run-input"
          cols="32"
          onChange={(e) => setLine(e.currentTarget.value)}
          placeholder="Enter some ezasm code..."
        />
        <button type="submit">Run!</button>
      </form>

      <p>{result}</p>
    </div>
  );
}

export default App;
