#!/usr/bin/env -S deno run --check

// @ts-types="../pkg/regtest.d.ts"
import { Main, type Writer, type TerminalSizeCallback } from "../pkg/regtest.js"


async function main() {
    using cleanup = new DisposableStack()
    
    Deno.stdin.setRaw(true)
    cleanup.defer(() => Deno.stdin.setRaw(false))

    const ui = new Main(size, out)
    cleanup.defer(() => ui.free())

    ui.render()
    
    const buf = new Uint8Array(256)
    while (true) {
        const bytesRead = await Deno.stdin.read(buf)
        if (bytesRead === null) {
            break
        }
        try {
            ui.push_stdin_bytes(buf.slice(0, bytesRead))
        } catch (_e) {
            // TODO: Update to only hide known exception.
            break
        }
    }
}

const size: TerminalSizeCallback= () => {
    return Deno.consoleSize()
}

const out: Writer = (bytes: Uint8Array) => {
    let written = Deno.stdout.writeSync(bytes)
    while (written < bytes.length) {
        const remainder = bytes.slice(written)
        written += Deno.stdout.writeSync(remainder)
    }
    return written
}

if (import.meta.main) {
    await main()
}