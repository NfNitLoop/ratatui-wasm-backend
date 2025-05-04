RegTest
=======

A RegEx tester for the terminal.

Mostly, this is a demo of the [ratatui-wasm-backend] crate.

The core app is written in Rust, and compiled to WASM, to run inside of Deno.

This lets it reach out to the V8 JavaScript runtime to do a real test of its `RegExp` implementation.

[ratatui-wasm-backend]: ../ratatui-wasm-backend/

Screen Recording
----------------

<https://asciinema.org/a/8Ljb2Tkp9SyujJpaDjMKBadGw>

Try it out
----------

```
deno run jsr:@nfnitloop/regtest
```

Install
-------

```
deno install --global jsr:@nfnitloop/regtest
```

To Do (Coming Soon?)
--------------------

 * CLI args to customize startup.
 * Better text editor. (Show cursor. Allow keyboard navigation)
 * Allow editing the "Text" box.
 * Allow configuring Regex flags.
 * RegExp syntax highlighting. 

More distant future:
 * Better support for (named/unnamed) match groups.