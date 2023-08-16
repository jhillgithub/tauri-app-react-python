import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [path, setPath] = useState("~");
  const [filesList, setFilesList] = useState("");

  async function listFiles() {
    try {
      const files = await invoke("list_files", { path });
      setFilesList(`Files in ${path}:\n${files}`);
    } catch (error) {
      console.error(error);
      setFilesList(`Error: ${error}`);
    }
  }

  function handlePathChange(event) {
    setPath(event.target.value);
  }

  return (
    <>
      <div>
        <label>
          Path:
          <input type="text" value={path} onChange={handlePathChange} />
        </label>
        <button onClick={listFiles}>List Files</button>
        <p>{filesList}</p>
      </div>
    </>
  );
}

export default App;
