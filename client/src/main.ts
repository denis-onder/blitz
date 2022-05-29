import App from "./App.svelte";

const target = document.getElementById("app");
target.parentElement.style.margin = "0";

const app = new App({
  target,
});

export default app;
