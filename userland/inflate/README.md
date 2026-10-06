# inflate

`nonos_inflate` decodes raw DEFLATE (RFC 1951), zlib (RFC 1950) and gzip
(RFC 1952) for userland. `no_std` with `alloc`, no dependencies. It is written
in this tree, not vendored.

## Public surface

Whole-stream calls return the output only when the stream decoded and its
checksum matched, and cap output at `MAX_OUT` (4 MiB):

- `inflate(src)`: raw DEFLATE. Bytes after the final block are ignored.
- `zlib(data)`: checks the header and the big-endian Adler-32.
- `gunzip(data)`: every member, concatenated, each checked against its CRC-32
  and ISIZE. Up to 64 members.
- `gunzip_within(data, limit)`: `gunzip` with the caller's limit in place of
  `MAX_OUT`.

Partial calls take a cap and return `Inflated { out, end, used }`, where `end`
is `End::Complete`, `Truncated`, `Capped` or `Corrupt` and `used` is the input
consumed. `Inflated::complete()` turns it back into an `Option`.

- `raw_partial(src, cap)`, `zlib_partial(data, cap)`,
  `gunzip_partial(data, cap)`.

For signed gzip files, `members(data)` and `members_within(data, limit)`
return each member's byte range and output as a `Member { start, end, body }`.
Unlike `gunzip`, they refuse the whole input if any byte belongs to no member.

## What it does not do

It does not compress. zlib streams with a preset dictionary (FDICT) or a
window over 32 KiB are refused. The gzip header CRC (FHCRC) is skipped, not
checked. In `gunzip` and `gunzip_partial`, bytes after a good member that do
not start another member are treated as trailing garbage and end the stream
without an error.

## Users

- `capsule_browser`: HTTP `Content-Encoding` (`src/browser/http/response/undo_coding.rs`)
  and WOFF fonts (`src/browser/fonts/woff.rs`).
- `capsule_linux`: splits APK packages and signed indexes into gzip members
  for their signature checks (`src/linux/install/auth/`), and unpacks gzip
  payloads (`src/linux/install/unpacked.rs`).
- `capsule_net_anon`: compressed directory documents
  (`src/directory/fetch/response.rs`).
- `capsule_terminal`: `nox pull` (`src/command/builtin/nox/pull/fetch.rs`).
- The proof crates `browser_http_proofs`, `capsule_browser_proofs`,
  `capsule_linux_proofs` and `image_paint_proofs`, which compile those
  callers' sources.

## Tests

The crate has no tests of its own and no Kani proofs. It is exercised by
`browser_http_proofs` (`src/inflate_tests.rs`, `src/partial_tests.rs`),
`capsule_browser_proofs` (`src/gzip_tests.rs`) and `capsule_linux_proofs`
(`src/tests/inflate_bound_tests.rs`), which `nix flake check` runs. See
[Browser](../../docs/handbook/apps/browser.md) and
[the proofs page](../../docs/handbook/verification/proofs.md).
