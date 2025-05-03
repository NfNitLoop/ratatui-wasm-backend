#!/usr/bin/env -S deno run --check

import { delay } from "jsr:@std/async@1.0.12"

import { Main } from "../pkg/wasmtest.js"


// main(Deno.consoleSize, (bytes: Uint8Array) => {
//     // console.log("bytes type:", typeof bytes)
//     Deno.stdout.writeSync(bytes)
// })

const size = () => {
    return Deno.consoleSize()
    // return {
    //     columns: 60,
    //     rows: 30,
    // }
}

const out = (bytes: Uint8Array) => {
    Deno.stdout.writeSync(bytes)
}
const ui = new Main(size, out)
ui.render()

const buf = new Uint8Array(256)
Deno.stdin.setRaw(true)
while (true) {
    const bytesRead = await Deno.stdin.read(buf)
    if (bytesRead === null) {
        break
    }
    try {
        ui.push_stdin_bytes(buf.slice(0, bytesRead))
    } catch (_e) {
        break
    }
}

ui.free()
