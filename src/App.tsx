import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { appWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [schemaFile, setSchemaFile] = useState("");
  const [input, setInput] = useState("");
  const [fixMsg, setFixMsg] = useState([]);

  listen(
    'SchemaFile', 
    (p) =>{
      setSchemaFile("Anything");
      invoke("ping");
    }
  );
  
  async function read_fix() {
    setFixMsg(await invoke("read_fix", { input }));
  }

  return (
    <div className="container">
      <h1>Fix Reader</h1>
      {/* <p>Schema: {schemaFile}</p> */}

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

      
      <ul style={{listStyle:'none'}}>
        {fixMsg.map(el => 
          <li key={el}>{el[0]}: {el[1]}</li>
        )}
      </ul>     
    </div>
  );
}

export default App;
