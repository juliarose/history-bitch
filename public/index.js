import init, { parse } from "/pkg/history_bitch.js";
import tradesResponse from "./response.json" assert { type: "json" };;

const containerEl = document.getElementById('container');

async function runWASM() {
  // Instantiate our wasm module
  const wasm = await init("/pkg/history_bitch_bg.wasm");
  // Call the function from wasm, save the result
  const response = tradesResponse;
  console.log(response);
  console.time('res');
  const result = parse(JSON.stringify(response));
  console.timeEnd('res');
  
  // Set the result onto the body
  containerEl.innerHTML = result;
};

runWASM().catch((e) => {
  console.log(e);
  containerEl.textContent = `error running wasm: ${e.message}`;
});
