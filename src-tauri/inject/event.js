window.addEventListener("DOMContentLoaded", (_event) => {
  const { invoke, event } = window.__TAURI__;
  event.emit("location", {
    href: location.href,
  });

  invoke("change_route", { href: location.href });
});
