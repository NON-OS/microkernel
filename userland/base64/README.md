# base64

`nonos_base64` is the RFC 4648 base64 used by userland capsules. `no_std` with
`alloc`, no dependencies.

## Public surface

- `encode(src) -> Vec<u8>`: standard alphabet (`+`, `/`) with `=` padding.
- `decode_b64(s) -> Option<Vec<u8>>`: accepts the standard and the URL-safe
  alphabet (`-`, `_`) in the same input. `=`, space, tab, CR and LF are skipped
  wherever they appear. Any other byte returns `None`.

The decoder is lenient. Padding is optional, and bits left over at the end are
dropped rather than rejected, so it does not check that the input is canonical
or that its length is valid.

## Users

- `capsule_browser` decodes `data:` URIs with `decode_b64`
  (`src/browser/image/data_uri.rs`).
- `capsule_terminal` encodes the credential in an `Authorization: Basic`
  header with `encode` (`src/command/builtin/nox/pull/auth.rs`).
- `image_paint_proofs` depends on it because it compiles the browser's image
  sources.

## Tests

The crate has no tests of its own. The `data:` URI path is exercised through
`image_paint_proofs`.
