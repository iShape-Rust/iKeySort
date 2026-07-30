import { readFile } from "node:fs/promises";

const bytes = await readFile(process.argv[2]);
const { instance } = await WebAssembly.instantiate(bytes, {});
const result = instance.exports.run_i64_32bit_boundary_regression();

if (result !== 0) {
    throw new Error(`wasm32 i64 regression failed with code ${result}`);
}

console.log("wasm32 i64 regression passed");
