import init, { parse } from "/pkg/history_bitch.js";
import tradesResponse from "/public/response.json" assert { type: "json" };;
import tradesResponseText from './trades.js';

const containerEl = document.getElementById('container');

async function runWASM() {
  // Instantiate our wasm module
  const wasm = await init("/pkg/history_bitch_bg.wasm");
  // Call the function from wasm, save the result
    const response = tradesResponse;
    console.log(tradesResponseText);
    console.time('parse');
    const parsed = JSON.parse(tradesResponseText);
    
    console.log(parsed);
    console.timeEnd('parse');
    console.time('res');
    const result = parse(tradesResponseText);
    console.timeEnd('res');
  
    // Set the result onto the body
    containerEl.innerHTML = result;
};

runWASM().catch((e) => {
  console.log(e);
  containerEl.textContent = `error running wasm: ${e.message}`;
});
