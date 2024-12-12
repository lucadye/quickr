import React, { useEffect, useState } from 'react';
import init, {
  text as qrFromText, valid_text,
  url  as qrFromUrl, valid_url
} from "wasm";

export default function Form() {
  // QR Settings
  const [mode, setMode] = useState("url");
  const [dataUrl, setDataUrl] = useState("");
  // Text/URL
  const [text, setText] = useState("");
  // Alert
  const [error, setError] = useState("");
  const [hint, setHint] = useState("");
  const clear = () => {
    setError("");
    setHint("");
  }

  useEffect(() => {
    init().then(() => {
      // Reset QR code if no input is provided.
      if (text === "") {
        setDataUrl("");
        clear();
        return;
      }
      let qr;
      switch (mode) {
        case "text":
          if (valid_text(text)) {
            qr = qrFromText(text, 0, "svg");
            setDataUrl(`data:image/svg+xml;utf8,${encodeURIComponent(qr)}`);
            clear();
          }
          else {
            setDataUrl("");
            setError("Plaintext cannot be a valid URL");
            setHint("Did you mean to select URL?");
          }
          break;
        case "url":
          if (valid_url(text)) {
            qr = qrFromUrl(text, 0, "svg");
            setDataUrl(`data:image/svg+xml;utf8,${encodeURIComponent(qr)}`);
            clear();
          }
          else {
            setDataUrl("");
            setError("URLs must begin with \"www.\", \"https://\" or \"http://\"");
            setHint("Did you mean to select text?");
          }
          break;
        default:
          return;
      }
    });
  }, [mode, text]);

  return (<>
    <form onSubmit={e => {
      e.preventDefault();
    }}>

      <label htmlFor="mode">QR Type</label>
      <select name="mode" id="mode" onChange={e => {
        e.preventDefault();
        let value = e.target.value;
        if (["url", "text"].includes(value)) {
          setMode(value);
          setDataUrl("");
          clear();
        }
      }}>
        <option value="url" >URL</option>
        <option value="text">Plaintext</option>
      </select>

      <label gtmlFor="text">{mode === "url" ? "URL" : mode === "text" ? "Text" : ""}</label>
      <input onChange={e => {
        e.preventDefault();
        setText(e.target.value);
      }} type="text" name="text" id="text" value={text}/>

      <div className="alert">
        <b>{error}</b>
        <i>{hint}</i>
      </div>
      {/*<button type="submit">Generate QR Code</button>*/}
    </form>
    {dataUrl ? <a href={dataUrl}><img src={dataUrl} alt="A QR code"/></a> : ""}
    {mode === "url" && dataUrl ? <a href={text}>{text}</a> : ""}
  </>);
}
