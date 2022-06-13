import init, { loadDemoApp } from "./demo_respo/pkg/demo_respo";

window.onload = () => {
  init().then(() => {
    loadDemoApp(".app");
  });
};
