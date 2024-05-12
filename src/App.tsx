import { useEffect, useState } from "react";
import { Store } from "tauri-plugin-store-api";

const store = new Store("settings.json");

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

  const handleSubmit:React.DOMAttributes<HTMLFormElement>["onSubmit"] = (e) => {
    e.preventDefault();
    console.log(e);
  };
  return (
    <div className="container">
      <form onSubmit={handleSubmit}>
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
        <div
          className="row"
          style={{
            justifyContent: "flex-end",
            gap: "1rem",
          }}
        ></div>
        <button type="submit" style={{ background: "#2da5ff", color: "#fff" }}>
          应用
        </button>
      </form>
    </div>
  );
}

export default App;
