import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { WebviewWindow } from "@tauri-apps/api/window";
import { emit, listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/api/dialog";

const App = () => {
  const [schemaFile, setSchemaFile] = useState("");
  const [separator, setSeparator] = useState("|");
  const [input, setInput] = useState("");
  const [convertedLines, setConvertedLines] = useState<FixMsg[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [counter, setCounter] = useState(0);

  useEffect(() => {
    invoke("get_schema_file").then((response) => {
      setSchemaFile(response as string);
    }).catch((_) => {
      setSchemaFile('not loaded');
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
      var fixMsg: FixMsg = await invoke("read_fix", { input, separator });
      console.log(fixMsg);
      fixMsg.id = counter;
      setConvertedLines([...convertedLines, fixMsg]);
      setCounter(counter + 1);
      setInput("");
    } catch (error) {
      setError(`Error: ${error}`);
    }
  }

  const clear = () => {
    setInput("");
    setConvertedLines([]);
  }

  const openWindow = async (msg: FixMsg) => {
    const label = `details_${msg.id}`;
    const window = WebviewWindow.getByLabel(label);
    if (window != null) {
      window.show();
      return;
    }

    new WebviewWindow(label, {
      url: 'details.html',
      width: 500,
    });

    listen('detailsInfoRequest', (req) => {
      if (req.windowLabel != label) return;
      emit('detailsInfoResponse', { line: msg })
    });
  }

  const removeLine = (id: number) => {
    let index = convertedLines.findIndex(x => x.id == id);
    if (index == -1) return;

    convertedLines.splice(index, 1);
    console.log("Removed at ", index);
    console.log("convertedLines: ", convertedLines);
    setConvertedLines([...convertedLines]);
  }

  const selectFileClick = async () => {
    const selected = await open({
      filters: [{
        name: 'Schema Fix',
        extensions: ['xml']
      }]
    })
    console.log("Selected: ", selected);
    if (selected !== null && typeof selected === 'string') {
      const items = selected.split('\\')
      setSchemaFile(items.pop() as string);
    }
  }

  // forms
  return (
    <div className="fix-reader-container">
      <h1 className="fix-reader-title">FixReader</h1>

      <div className="error-section" hidden={error == null}>
        <span>{error}</span>
      </div>

      <div className="schema-section">
        <label htmlFor="schemaFile">Schema File:</label>
        <button onClick={selectFileClick}>{schemaFile}</button>

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
            <tr key={msg.id}>
              <td className="fixLine" onClick={() => openWindow(msg)}>
                {
                  msg.values
                    .map(field => field.tag + ": " + field.value).join(" | ")
                }
              </td>
              <td>
                <button className="removeLine" onClick={() => removeLine(msg.id)}>X</button>
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