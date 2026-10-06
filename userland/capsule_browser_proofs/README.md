# capsule_browser_proofs

Host test crate for the browser engine in `userland/capsule_browser`. It compiles the engine's
module trees through `#[path]` and runs them against known answers, fixtures in `fixtures/`
(fonts and images) and scripted network input, with no GUI. Its `harness` feature (on by
default) is what makes the engine's direct cascade entry reachable (`Cargo.toml`). It links
`nonos_toolkit`, `nonos_app_skeleton`, `nonos_inflate`, `nonos_brotli` and `tls_proofs`.

## What is under test

`src/browser/mod.rs` mounts the capsule's real module roots from
`../../../capsule_browser/src/browser/`: `css/`, `dom/`, `fonts/`, `html/`, `layout/`,
`omnibox/`, `url/`, `http/`, `event/dom_print.rs`, `net/recv_pending.rs` and
`fetch/enqueue_css/sheet.rs`. Narrower mirrors pick single files:

- `src/browser/fetch/mod.rs`: the fetch machine's pure half (`fetch/connect.rs`,
  `deadline.rs`, `pool/`, `tls/`, `socks/`, `run.rs` and others).
- `src/browser/image/mod.rs`: decode, ICO, JPEG, sniffing and the image store, plus
  `userland/base64/src/decode.rs`.
- `src/browser/net/`: `drain.rs`, `parse_ipv4.rs`, `recv_kind.rs`, and from `mixnet/`
  `choice.rs`, `refusal.rs`, `way.rs`, `conv.rs`, `frames.rs`, `pace.rs`, `fault.rs`,
  `streams.rs`.
- `src/browser/js/interp/mod.rs`: only `js/interp/attr_prop.rs`.

TLS comes from `tls_proofs` (`pub use tls_proofs as tls13`). `src/browser/manifest.rs` is a
local stand-in carrying only the viewport size (1360 by 760). `src/render.rs` drives parse,
cascade, box tree and layout in the capsule's call order.

## What the tests check

Unit modules are listed in `src/lib.rs`; integration tests are in `tests/`.

- CSS: specificity, selectors, cascade layers, variables, at-rules (`cascade_tests.rs`,
  `selector_tests.rs`, `src/cascade_proofs/`, `tests/selector_*.rs`), including hostile
  selectors that once aborted or stalled the capsule (`tests/selector_hostile.rs`).
- Layout: grid, flex, tables, floats, positioning, bidi (`grid_tests.rs`, `table_tests.rs`,
  `src/layout_tests/`), and `hostile_geom_tests.rs`
  (`deep_absolute_and_fixed_nests_lay_out`, `absurd_lengths_and_math_stay_bounded`).
- Fonts and images: WOFF2, JPEG, SVG, image store (`src/color_fonts_images/`,
  `tests/glyph_cache.rs`, `tests/glyph_heap.rs`).
- URL and HTTP: RFC 3986 joins, chunked and gzip decoding (`url_tests.rs`,
  `chunked_tests.rs`, `gzip_tests.rs` with `decodes_every_member_in_order`).
- Fetch machine with scripted sockets and real TLS records (`src/fetch_proofs/`):
  `https_sends_its_client_hello_in_the_call_the_connect_completes`,
  `a_verification_failure_is_never_retried`, `six_to_one_host_and_eight_in_all`.
- UI state: focus, history, scroll, line editing (`focus_tests.rs`, `history_tests.rs`).

## Running

```sh
cd userland/capsule_browser_proofs
cargo test --release
```

At the time of writing this runs 347 unit tests plus 81 in `tests/`. CI runs it through
`nix flake check` (the `verify.yml` flake job): `tools/nix/checks.nix` takes every
`userland/*_proofs` crate with a `Cargo.lock`, runs `cargo test --release` with overflow checks
on, and then `cargo clippy --release -- -D warnings` over the library only, since this crate is
in the `lintLib` list whose tests are not lint-clean yet.

## Not covered

The HTML parser has a dedicated suite in `userland/capsule_browser_html_proofs`. The JS
interpreter beyond `attr_prop.rs`, real socket and DNS syscalls, the compositor surface, input
delivery and on-target painting are not exercised. `src/lib.rs` allows three clippy lints
(`redundant_closure`, `manual_is_multiple_of`, `too_many_arguments`) for the mounted source.
