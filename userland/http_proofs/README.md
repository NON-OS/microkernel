# http_proofs

Host-runnable proofs for `nonos_http`'s response parser. The library is pulled
in by path, so the parser under test is the one that ships.

- `status_line`: `HTTP/1.x`, three digits from 100 to 599, then a space or the
  line end (RFC 9112 4).
- `fields`: field names are tokens with no whitespace before the colon, and the
  field count and head size are bounded.
- `body_length`: Content-Length is digits only and every stated value must
  agree; Transfer-Encoding frames the body by its last coding (RFC 9112 6.3).
- `chunked`: the CRLF after each chunk is checked, whitespace after a size is
  allowed, extensions are ignored.
- `interim`: 1xx responses before the final one are skipped, and too many are
  refused.
- `noise`: random bytes never panic the parser.

Run: `cd userland/http_proofs && cargo test --release`. `nix flake check` runs it as
`proofs-http_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`.

See [the network stack](../../docs/handbook/network/stack.md) and
[proofs](../../docs/handbook/verification/proofs.md).
