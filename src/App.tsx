import { useEffect, useState } from "react";
import { Store } from "@tauri-apps/plugin-store";

const store = new Store(".settings.dat");

function App() {
  const [css, setCss] = useState("");
  const [fontFamily, setFontFamily] = useState("");

  useEffect(() => {
    (async () => {
      const fontFamily = (await store.get<string>("font-family")) ?? "";
      setFontFamily(fontFamily);
      const css = (await store.get<string>("css")) ?? "";
      setCss(css);
    })();
  }, []);

  const handleSave = (key: string, callback: (val: string) => void) => {
    return async (
      e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
    ) => {
      const value = e.target.value;
      callback(value);
      await store.set(key, value);
      await store.save();
    };
  };
  return (
    <div className="container">
      <div className="row">
        <label>字体</label>
        <input
          value={fontFamily}
          onChange={handleSave("font-family", setFontFamily)}
        />
      </div>
      <div className="row">
        <label>CSS片段</label>
        <textarea value={css} onChange={handleSave("css", setCss)} />
      </div>
    </div>
  );
}

export default App;
