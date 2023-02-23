import {invoke} from "@tauri-apps/api";
import React from "react";
import ReactDOM from "react-dom/client";


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <div style={{
      position: "fixed",
      left: "0",
      top: "0",
      display: "flex",
      height: "100vh",
      width: "100vw",
      justifyContent: "center"
    }}>
      <video style={{height: "100%"}} src={"http://localhost:8080/1655780442153_540.mp4"} autoPlay loop muted/>
    </div>
  </React.StrictMode>
);


window.setTimeout(() => {
  invoke("update_location").then()
}, 0)