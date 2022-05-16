import init, { loadDemoApp } from "./pkg/respo";

window.onload = () => {
  init().then(() => {
    console.log("loaded");

    loadDemoApp();
  });
};
