# attest_receipt

The host-side reader for a NØNOS attestation receipt. A machine hands out a
32-byte registry root under a TPM signature and the capsule entries behind it.
This crate refolds the entries the way the kernel does, compares the result
with the signed root, and on a match prints what each capsule measured, which
capabilities it held and who signed it.

It builds for the host, not for NØNOS. It depends on `blake3` with default
features off.

## Kernel sources it compiles

The format is not restated. `src/kernel/` pulls the kernel's own files in by
`#[path]`:

- `src/security/attest_registry/format.rs` for `DOMAIN`
  (`nonos.attest.registry.v1`) and `ENTRY_LEN` (45 bytes: pid, measurement,
  capability mask, authority).
- `src/capabilities/types/{defs,table,as_str,guard}.rs` for `Capability`, its
  bit table and its names.

A change to the fold or a renumbered capability breaks this build. The one
thing restated is the authority byte (`Vendor` 0, `Publisher` 255,
`Developer(s)` 1 + s), in `src/decode/authority.rs`, because the kernel's
`Authority` type cannot be included on its own.

## Public surface

- `fold_root(entries)`: BLAKE3 over `DOMAIN`, the entry count as a big-endian
  `u32` (byte length divided by `ENTRY_LEN`), then the entry bytes.
- `verify_root(entries, signed)`: `Ok(())` when the fold equals `signed`,
  otherwise `RootMismatch` with both values.
- `parse_entries(bytes)`: splits into `Entry { pid, measurement, caps,
  authority }`. A length that is not a multiple of 45 is
  `ParseError::Ragged`. It verifies nothing.
- `capability_names(mask)`, `reaches_network(mask)`, `Authority`: read the
  mask and authority byte. Bits the kernel table does not define are reported
  as `unrecognised bits`.
- `render(entries)`: the table the binary prints, with a network column and a
  count by authority.
- `parse_root(hex)`: 64 hex characters with an optional `0x`. Anything else is
  `None`; it does not pad or truncate.
- `parse_args`, `Mode`, `USAGE`: the command line.

## The binary

```text
nonos-receipt <entries file> <signed root, hex>
```

It prints `VERIFIED` and the table and exits 0 when the fold matches, prints
`REFUSED` and exits 1 when it does not or the file is ragged, and exits 2 on bad
usage or an unreadable file.

## What it does not do

It does not check the TPM signature or quote. The caller verifies that and
passes in the root the signature covers. It does not take the root out of a
document itself, and on a mismatch it does not say which entry differs.

## Users

No crate depends on it. It is the `nonos-receipt` tool.

## Tests

23 unit tests under `src/tests/` and `src/hexcode.rs`: the fold matches the
kernel's fold, the count and domain are bound, a widened mask, dropped entry,
reordered set, changed authority or swapped measurements is caught, a
truncated set is refused, and the capability names come from the kernel table.
`nix flake check` runs them as `proofs-attest_receipt`, since the crate is
listed by name in `tools/nix/checks.nix`. There are no Kani proofs.

See [TPM](../../docs/handbook/trust/tpm.md) for the attestation key and quotes.
