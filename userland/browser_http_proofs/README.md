# browser_http_proofs

Host test crate for the browser's HTTP response path. It compiles modules of
`userland/capsule_browser` through `#[path]`, at the paths the capsule gives
them, and runs them against known answers and recorded responses, with no
boot and no network. Its one dependency is `nonos_inflate` (`Cargo.toml`). The
browser itself is described in
[docs/handbook/apps/browser.md](../../docs/handbook/apps/browser.md).

## What is under test

`src/browser/mod.rs` mounts:

- `capsule_browser/src/browser/http/` and `url/`: request and response
  parsing, framing, chunked transfer, content codings, charsets.
- `fetch/apply_css/css_cut.rs` and `css_fold.rs`: folding a fetched sheet into
  the page CSS and deciding when the page lays out again.

`vectors/` holds the inputs: 54 synthetic RFC 9110 and RFC 9112 cases listed
in `vectors/cases.txt` with the outcome each must give, pages in legacy
encodings under `vectors/encodings/`, and a gzip vector (`kernel.gz`) with its
expected output.

## What the tests check

34 `#[test]` functions in eleven test modules, listed in `src/lib.rs`:

- `cases_tests.rs`, `framing_tests.rs`, `chunked_tests.rs`, `keepalive_tests.rs`:
  every conformance case parses as listed, and framing agrees with parsing on
  each; chunk sizes, extensions and trailers; HTTP/1.0 and `Connection: close`
  do not keep the connection.
- `inflate_tests.rs`, `partial_tests.rs`: gzip and deflate decoding, a page cut
  short keeps what arrived, images and scripts are whole or nothing, a large
  page is cut at the inflate cap, and a decompression bomb is refused.
- `charset_tests.rs`, `sniff_tests.rs`, `prescan_tests.rs`: the header, then a
  `<meta>` within the first 1024 bytes, then a guess decide the encoding; a
  byte order mark wins and is removed.
- `css_fold_tests.rs`: a page lays out once whatever its sheets do, and an
  oversized sheet is cut after a whole rule.

## Running

```sh
cd userland/browser_http_proofs
cargo test --release
```

CI runs it through `nix flake check`, which `tools/nix/checks.nix` builds over
every `userland/*_proofs` crate with a `Cargo.lock`: the tests in release mode
with overflow checks on, then clippy over all targets with `-D warnings`.

## Not covered

The socket and DNS calls, TLS, the fetch state machine and the DOM are not
mounted here; `userland/capsule_browser_proofs` covers the fetch machine and
TLS records, `userland/capsule_browser_html_proofs` the HTML parser.
