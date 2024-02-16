import init, {Table} from "./pkg/rtable.js"

const runWasm = async () => {
    // Instantiate our wasm module
    await init("./pkg/rtable_bg.wasm");
    const tt = new Table();
    console.log(tt.rows(0, 25))
    globalThis.tt = tt;

    globalThis.tt.rows(0, 2500).forEach(it => {
        const tr = document.createElement('tr');

        let td = document.createElement('td')
        td.appendChild(document.createTextNode(it.Mercury))
        tr.appendChild(td);

        td = document.createElement('td')
        td.appendChild(document.createTextNode(it.Venus))
        tr.appendChild(td);

        td = document.createElement('td')
        td.appendChild(document.createTextNode(it.Mars))
        tr.appendChild(td);

        td = document.createElement('td')
        td.appendChild(document.createTextNode(it.Earth))
        tr.appendChild(td);

        document.getElementById('main-table').appendChild(tr);
    });
};

await runWasm();
