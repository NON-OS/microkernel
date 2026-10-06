# nonos_recon

`nonos_recon` is the engine for a TCP connect scan: parse a target and a port
list, probe each port through a caller-supplied `Probe`, and format the
result. Everything here is pure. The network connect is the caller's. `no_std`
with `alloc` outside its own tests, no dependencies.

## Public surface

- `parse_target(s) -> Result<Target, TargetError>`: a dotted-quad IPv4
  literal only. Each octet is 1 to 3 digits, 0 to 255. Errors are `Shape`
  (not four parts) and `Octet`.
- `parse_ports(spec) -> Result<Vec<u16>, PortError>`: comma-separated ports
  and `start-end` ranges, such as `22,80,443` or `1-1024`, with spaces around
  fields tolerated. The result is sorted and deduplicated. Ports must be 1 to
  65535. Errors are `Malformed`, `OutOfRange`, `Backwards` and `TooMany`. The
  expansion is capped at `MAX_PORTS` (65,535) before deduplication, so
  overlapping ranges that add up to more than that are refused rather than cut.
- `trait Probe { fn probe(&mut self, target: &Target, port: u16) -> PortState }`
  and `scan(target, ports, probe) -> Vec<ScanRow>`: one probe per port, in the
  order given.
- `PortState` (`Open`, `Closed`, `Filtered`), `ScanRow { port, state }`, and
  `format_report(octets, rows)`: one line per open or closed port, then a
  count line. Filtered ports are counted, not listed.

## What it does not do

No sockets, no timeouts, no DNS and no IPv6. It does not scan in parallel.

## Users

No crate in this tree depends on it. `recon` is a priced tool id in
`nonos_nox_license` (`src/price.rs`), but no capsule here implements `Probe`
or calls `scan`.

## Tests

Eleven unit tests in `src/tests.rs` cover target and port parsing, the full
range, and a scan and report driven by a table of canned outcomes. Run them
with `cargo test`. The crate is not a `*_proofs` crate, so `nix flake check`
does not run them. No Kani proofs. The pay-per-use tool licences that name
`Recon` are covered in [Wallet](../../docs/handbook/apps/wallet.md).
