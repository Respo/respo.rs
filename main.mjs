import init, { renderElement } from "./pkg/respo";

window.onload = () => {
  init().then(() => {
    console.log("loaded");
    renderElement();
  });
};
