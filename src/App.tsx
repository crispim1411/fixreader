import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [input, setInput] = useState("");
  const [fixMsg, setFixMsg] = useState([]);
  
  async function read_fix() {
    setFixMsg(await invoke("read_fix", { input }));
  }

  return (
    <div className="container">
      <h1>Fix Reader</h1>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          read_fix();
        }}>
        <input
          id="fix-input"
          onChange={(e) => setInput(e.currentTarget.value)}
          placeholder="Paste the fix message here..."
        />
        <button type="submit">Convert</button>
      </form>

      {fixMsg.map(el => 
        <p>{el[0]}: {el[1]}</p>
      )}
    </div>
  );
}

export default App;
