import init, {Table} from "./pkg/rtable.js"

const runWasm = async () => {
    // Instantiate our wasm module
    await init("./pkg/rtable_bg.wasm");
    const tt = new Table();

    globalThis.tt = tt;
    // Set the result onto the body
    document.body.textContent = `Hello World! addResult: ${tt.len()}`;
};
runWasm();