import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

const App = () => {
  const [schemaFile, setSchemaFile] = useState("");
  const [separator, setSeparator] = useState("^");
  const [input, setInput] = useState("");
  const [convertedLines, setConvertedLines] = useState<string[]>([]);

  useEffect(() => {
    invoke("get_schema_file").then((response) => {
      console.log('schema file');
      setSchemaFile(response as string);
    });
  });

  const readFix = async () => {
    console.log('reading fix');
    var result: string[][] = await invoke("read_fix", { input, separator });
    var fixMsg = result.map((x) => x.join(": ")).join(", ")
    setConvertedLines(convertedLines.concat(fixMsg));
  }

  return (
    <div className="fix-reader-container">
      <h1 className="fix-reader-title">FixReader</h1>
      <div className="schema-section">
        <label htmlFor="schemaFile">Schema File:</label>
        <input
          type="text"
          id="schemaFile"
          value={schemaFile}
          disabled
        />
        <label htmlFor="separator">Separator:</label>
        <input
          type="text"
          id="separator"
          value={separator}
          onChange={(e) => setSeparator(e.target.value)}
        />
      </div>
      <form 
        className="input-section"
        onSubmit={(e) => {
          e.preventDefault();
          setConvertedLines([...convertedLines, input])
          console.log("set input");
        }}>
        <label htmlFor="message">Message:</label>
        <input
          type="text"
          id="message"
          value={input}
          onChange={(e) => setInput(e.currentTarget.value)}
        />
        <button onClick={(e) => {
          e.preventDefault();
          readFix();
          console.log("click");
        }}>Convert</button>
        <button onClick={(e) => {
          e.preventDefault();
          setInput("");
          setConvertedLines([]);
        }}>Clear</button>
      </form>
      <table className="converted-table">
        <thead>
          <tr>
            <th>Converted Lines</th>
          </tr>
        </thead>
        <tbody>
          {convertedLines.map((line) => 
            (
            <tr>
              <td>{line}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default App;
