import fs from "fs"

const mathwasm = fs.readFileSync("./math.wasm");


// calling web assembly into js.
const math = await WebAssembly.instantiate(
    new Uint8Array(mathwasm)).then(res => res.instance.exports);

// console.log(mathwasm);
{/* <Buffer 00 61 73 6d 01 00 00 00 01 06 01 60 01 7f 01 7f 03 02 01 00 07 0a 01 06 73 71 75 61 72 65 00 00 0a 09 01 07 00 20 00 20 00 6c 0b> */}
console.log(math.square(50));