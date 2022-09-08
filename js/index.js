const wasm = import('../pkg');
wasm
 .then(m => { m.entry_point() })
 .catch(console.error);