import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

import MessageList from "./messageList/messageList";
import Form from "./form/form";

const App = () => {
  const [convertedLines, setConvertedLines] = useState<FixMsg[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [counter, setCounter] = useState(0);
  
  const handleError = (error: string) => {
    setError(`Error: ${error}`);
  }

  const clear = () => {
    setConvertedLines([]);
  }

  const readFix = async (input: string) => {
    var fixMsg: FixMsg = await invoke("read_fix", { input });
    fixMsg.id = counter;
    setCounter(counter + 1);
    setConvertedLines([...convertedLines, fixMsg]);
  }

  const removeLine = (id: number) => {
    let index = convertedLines.findIndex(x => x.id == id);
    if (index == -1) return;

    convertedLines.splice(index, 1);
    setConvertedLines([...convertedLines]);
  }
  
  useEffect(() => {
    if (error == null) return;
    const timer = setTimeout(() => {
      setError(null);
    }, 5000);
    return () => clearTimeout(timer);
  }, [error]);

  return (
    <div className="fix-reader-container" style={{paddingBottom: "100%", overflow: "hidden"}}>
      <h1 className="fix-reader-title">FixReader</h1>
      <div className="error-section" hidden={error == null}>
        <span>{error}</span>
      </div>
      <Form 
        handleError={handleError} 
        handleReadFix={readFix} 
        handleClear={clear} />
      <MessageList 
        lines={convertedLines} 
        handleRemoveLine={removeLine} />
    </div>
  );
}

export default App;