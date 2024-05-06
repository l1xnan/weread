window.addEventListener("DOMContentLoaded", (_event) => {
  // https://github.com/tauri-apps/tauri/issues/8476
  window.__TAURI__.event.emit("location", {
    href: location.href,
  });
});
