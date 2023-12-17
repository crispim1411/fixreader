import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { WebviewWindow } from "@tauri-apps/api/window";
import { emit, listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/api/dialog";
import Details from "./Details";

const App = () => {
  const [schemaFile, setSchemaFile] = useState("");
  const [separator, setSeparator] = useState("|");
  const [input, setInput] = useState("");
  const [convertedLines, setConvertedLines] = useState<FixMsg[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [counter, setCounter] = useState(0);
  const [detailWindow, setDetailWindow] = useState<WebviewWindow | null>(null);
  const [hide, setHide] = useState<Map<number, boolean>>(new Map());

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
      setCounter(counter + 1);
      setConvertedLines([...convertedLines, fixMsg]);
      setInput("");
      setHide(map => new Map(map.set(fixMsg.id, false)))
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

    detailWindow?.close();
    setDetailWindow(new WebviewWindow(label, {
        url: 'details.html',
        width: 500,
      })
    );

    listen('detailsInfoRequest', (req) => {
      if (req.windowLabel != label) return;
      emit('detailsInfoResponse', { line: msg })
    });
  }

  const removeLine = (id: number) => {
    let index = convertedLines.findIndex(x => x.id == id);
    if (index == -1) return;

    convertedLines.splice(index, 1);
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
      try {
        await invoke("set_schema_file", { path: selected });
        setSchemaFile(selected);
      } catch (error) {
        setError(`Error: ${error}`);
      }
    }
  }

  // forms
  return (
    <div className="fix-reader-container">
      <h1 className="fix-reader-title">FixReader</h1>

      <div className="error-section" hidden={error == null}>
        <span>{error}</span>
      </div>

      <div>
        <label htmlFor="separator">Separator:</label>
        <input
          type="text"
          className="separator-input"
          maxLength={3}
          value={separator}
          onChange={(e) => setSeparator(e.target.value)}
        />
      </div>

      <div>
        <label htmlFor="schemaFile">Schema:</label>
        <button onClick={selectFileClick}>{schemaFile}</button>
      
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
            <><tr key={msg.id}>
              <td className="fixLine" 
                onClick={(_) => setHide(map => new Map(map.set(msg.id, !map.get(msg.id))))}>
                {
                  msg.values
                    .map(field => field.tag + ": " + field.value).join(" | ")
                }
              </td>
              <td>
                <button className="removeLine" onClick={() => removeLine(msg.id)}>X</button>
              </td>
            </tr>
            <div className="collapsible" onClick={(_) => setHide(map => new Map(map.set(msg.id, !map.get(msg.id))))} style={{"height": hide.get(msg.id) ? "auto" : 0, "overflow": "clip"}}>
                <Details line={msg}/>
            </div></>
          ))
        }
        </tbody>
      </table>
    </div>
  );
}

export default App;