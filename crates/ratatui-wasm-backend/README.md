ratatui-wasm-backend
====================

A `Backend` implementation for [Ratatui] targeting WASM.

[RataTUI]: https://ratatui.rs

The goals of this implementation are:

 * A simple implementation of the Ratatui `Backend` trait which can run in WASM.
 * Can run in the browser, or the command-line.
 * Uses standard ANSI codes for controlling the terminal. (You'll need a modern terminal app.)

Known issues: 
 * Probably not the most efficient at the codes that it outputs.
 * Not yet well tested. But I'm open to bug reports & PRs!
 * When exiting 

Usage
-----

The basic process is:
 * Create a (ratatui-wasm-backend) `AnsiBackend` instance. This currently needs 2 arguments:
   * get_size - a callback to get the size of the terminal window
   * stdout_writer - a place to synchronously write bytes to stdout.
 * Call its `.exclusive()` method.
 * Create the Ratatui `Terminal`, passing it the above backend.
 * Start your event loop:
   * Process event
   * update application state
   * re-render your TUI
 * on exit
   * end the "exclusive" (alternate) terminal mode.  (`AnsiBackend.normal()`)

### Recommendations 

Process stdin in Rust.

You *can* do it in JavaScript. But for every type of event you need to send across the
JavaScript/Rust boundary, you'll potentially need to make a new typescript type, or a new 
wasm-bindgen method. I just start up an async loop in JavaScript to grab bytes from stdin (Don't forget to set "raw mode"!), and shove bytes into my Rust app for it to handle & rerender.

Example
-------

This repository contains example code that uses this backend. See:

[../regtest](../regtest)

Or have a look at the [screen recording](https://asciinema.org/a/8Ljb2Tkp9SyujJpaDjMKBadGw)


Future / To Do
--------------

 * More efficient terminal code output.
 * Some unit tests? 😅
 * Move more of the JavaScript setup/teardown into Rust, to reduce boilerplate.
 * Link to Rust docs once this crate is published.