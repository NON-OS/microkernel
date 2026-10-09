# nonos_qjs

`nonos_qjs` embeds the QuickJS-ng JavaScript engine as a `no_std` Rust crate
for the browser capsule, its only user (`userland/capsule_browser/Cargo.toml`).
It is a library, not a capsule: it has no `Capsule.mk`, no service and no
capability word of its own, and it runs with the browser's authority. How the
browser uses it is in
[docs/handbook/apps/browser.md](../../docs/handbook/apps/browser.md);
[DESIGN.md](DESIGN.md) sets out the layers and the freestanding fixes in more
detail.

## What is in it

- `vendor/`: the QuickJS-ng C core (`quickjs.c`, `libregexp.c`,
  `libunicode.c`, `dtoa.c`), the eval shim and `dom_bindings.c`, which binds
  `document`, elements, events, timers, `location`, `history` and storage to
  `njs_dom_*` callbacks the browser implements over its own node tree.
- `shim/`: headers that declare, and do not implement, the libc symbols the
  core references.
- `src/engine/`: the safe Rust surface, `Engine`. `new` creates a runtime and
  context, `install_dom` hands it the page's DOM pointer, `eval` runs source,
  `dispatch_event` fires listeners (`dispatch_key` and `dispatch_press`
  with a key's names and modifiers, or a press's position and button),
  `default_prevented` reports
  `preventDefault`, `flush_timers` runs due timers, and `take_navigation`
  returns a navigation a script asked for, `take_history_step` a step
  through the reader's history (`history.back()`, `forward()`, `go(n)`),
  and `take_dialog` the `alert`, `confirm` or `prompt` a script called
  (answered at once: nothing, false, null) for the browser to say.
- `src/alloc_stubs.rs`, `src/math_stubs.rs`, `src/str_stubs.rs`,
  `src/misc_stubs.rs`: the C symbols the core links against, from the Rust
  allocator, the `libm` crate and small Rust routines.

## Build

`build.rs` compiles the C core with clang for the bare target of the capsule's
architecture (`x86_64-unknown-none` for the browser), freestanding, PIC, with
`-nostdinc` and the `shim/` headers. The `hosted` feature instead builds it
against the system libc and leaves the stub modules out, for the browser's
render test build on a host.

## Tests

`tests/prelude/run.mjs` extracts the JavaScript prelude from
`vendor/dom_bindings.c` and runs its checks for `classList`, `history`,
`location`, `dataset`, events, storage and timers under Node.js.
`cargo test --release --features hosted` runs the real engine on the host:
`tests/limits.rs` (time, heap and stack limits), `tests/clocks.rs` (the lent
clocks behind `Date` and `performance`) and `tests/page.rs` (the bindings
over a stand-in host: load events, cookies, `el.style`, `el.value`,
`location`, history, `matchMedia`, scroll and bubbling). None of the
browser's proof crates links it.

## What it does not do

- No `fetch`, `XMLHttpRequest` or `WebSocket` binding: a script has no network.
- `localStorage` and `sessionStorage` are a plain object that lasts as long as
  the page.
- `eval` returns a result and an exception message alike as text, so a caller
  cannot tell whether a script threw.
- No WebAssembly, WebGL or workers.
