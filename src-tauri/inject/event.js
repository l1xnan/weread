window.addEventListener("DOMContentLoaded", (_event) => {
  window.__TAURI__.event.emit("location", {
    href: location.href,
  });
});
