import init, { loadDemoApp } from "./pkg/respo";

window.onload = () => {
  init().then(() => {
    loadDemoApp(".app");
  });
};
