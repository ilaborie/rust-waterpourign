import {solve} from "waterpouring-wasm";

const waterpouring = document.getElementById("waterpouring");
const output = document.querySelector("output");

waterpouring.addEventListener('submit', event => {
  event.preventDefault();
  const result = solve(waterpouring.from.value, waterpouring.to.value);
  console.log({result});
  output.textContent = JSON.stringify(result, null, 2);
  return false;
});
