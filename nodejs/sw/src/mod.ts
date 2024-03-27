self.addEventListener("install", (e) => {
  console.log("[Service Worker] Install");
});

self.addEventListener("activate", (e) => {
  console.log("[Service Worker] Activate");
});

// do nothing, default browser behavior occurs
addEventListener("fetch", (e) => {
  console.log("[Service Worker] Fetch!");
});
