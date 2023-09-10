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
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke("get_schema_file").then((response) => {
      setSchemaFile(response as string);
    });
  }, []);

  useEffect(() => {
    if (error == null) return;
    const timer = setTimeout(() => {
      setError(null);
    }, 5000);
    return () => clearTimeout(timer);
  }, [error]);

  const readFix = async (e: any) => {
    e.preventDefault();
    if (input.length == 0) return;
    try {
      var fixMsg: FixMsg[] = await invoke("read_fix", { input, separator });
      setConvertedLines(convertedLines.concat(fixMsg));
    } catch (error) {
      setError(error as string);
    }
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
      console.log("sending:", line);
      emit('detailsInfoResponse', { line: line })
    });
  }

  // forms
  return (
    <div className="fix-reader-container">
      <h1 className="fix-reader-title">FixReader</h1>

      <p>{error}</p>

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
          convertedLines.map((msg, index) => (
            <tr key={index}>
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