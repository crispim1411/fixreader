import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import { useEffect, useState } from "react";
import "./form.css";

interface FormProps {
    handleReadFix: (input: string[]) => void,
    handleError: (error: string) => void,
    handleClear: () => void
  }
  
const Form = ({handleReadFix, handleError, handleClear} : FormProps) => {
    const [schemaFile, setSchemaFile] = useState("");
    const [input, setInput] = useState("");
  
    useEffect(() => {
      loadSchema();
    }, []);
  
    const loadSchema = async () => {
      try {
        var response = await invoke("get_schema_file");
        setSchemaFile(response as string);
      } catch (error) {
        setSchemaFile('not loaded')
        handleError(error as string);
      }
    }
  
    const setSchema = async (path: string) => {
      try {
        await invoke("set_schema_file", { path });
        setSchemaFile(path);
      } catch (error) {
        handleError(error as string);
      }
    }
  
    const onClick = async (e: any) => {
      e.preventDefault();
      if (input.length == 0) return;
      try {
        await handleReadFix([input]);
      } catch (error) {
        handleError(error as string);
      }
    }
  
    const selectFileClick = async () => {
        const selected = await open({
            filters: [{
                name: 'Schema Fix',
                extensions: ['xml']
            }]
        })
      if (selected !== null && typeof selected === 'string') {
        await setSchema(selected);
      }
    }

    const handleClipboard = async () => {
      const text = await navigator.clipboard.readText();
      if (text == "") {
        handleError("No message on clipboard");
      } 
      try {
        await handleReadFix(text.split("\n"));
      } catch (error) {
        handleError(error as string);
      }
    }
  
    return (
      <>
      <div>
        <label htmlFor="schemaFile">Schema:</label>
        <button onClick={selectFileClick}>{schemaFile}</button>
      </div>
      <form
          className="input-section"
          onSubmit={onClick}>
          <label htmlFor="message">Message:</label>
          <input
            type="text"
            id="message"
            value={input}
            onChange={(e) => setInput(e.currentTarget.value)}
          />
          <button type="submit">Convert</button>
          <button onClick={handleClipboard}>Paste Clipboard</button>
          <button onClick={handleClear}>Clear</button>
        </form>
      </>
    )
}
export default Form;