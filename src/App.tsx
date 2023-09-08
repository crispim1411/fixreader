import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { WebviewWindow } from "@tauri-apps/api/window";
import { emit, listen } from "@tauri-apps/api/event";

const App = () => {
  const [schemaFile, setSchemaFile] = useState("");
  const [separator, setSeparator] = useState("^");
  const [input, setInput] = useState("");
  const [convertedLines, setConvertedLines] = useState<FixMsg[]>([]);

  useEffect(() => {
    invoke("get_schema_file").then((response) => {
      console.log('schema file');
      setSchemaFile(response as string);
    });
  }, []);

  const readFix = async (e: any) => {
    e.preventDefault();
    if (input.length == 0) return;

    console.log('reading fix');
    var fixMsg: FixMsg[] = await invoke("read_fix", { input, separator });
    setConvertedLines(convertedLines.concat(fixMsg));
  }

  const clear = (e: any) => {
    e.preventDefault();
    setInput("");
    setConvertedLines([]);
  }

  const openWindow = async (line: FixMsg) => {
    const webview = new WebviewWindow('details', {
      url: 'details.html',
    });

    listen('detailsInfoRequest', () => {
      console.log("sending", line);
      emit('detailsInfoResponse', { line: line})
    });
  }

  // forms
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
        onSubmit={readFix}>
        <label htmlFor="message">Message:</label>
        <input
          type="text"
          id="message"
          value={input}
          onChange={(e) => setInput(e.currentTarget.value)}
        />
        <button type="submit">Convert</button>
        <button onClick={clear}>Clear</button>
      </form>
      <table className="converted-table">
        <thead>
          <tr>
            <th>Converted Lines</th>
          </tr>
        </thead>
        <tbody>
        {
          convertedLines.map(msg => (
            <tr>
              <td onClick={() => openWindow(msg)}>
                {
                  msg.fields
                    .map(field => field.tag + ": " + field.value).join(" | ")
                }
              </td>
            </tr>
          ))
        }
        </tbody>
      </table>
    </div>
  );
}

export default App;