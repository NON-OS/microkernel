# nonos_bt_wire_proofs

Host tests for `nonos_bt_wire`. The crate depends on `nonos_bt_wire` by path
and calls its public API. Everything in `src/` is `#[cfg(test)]`.

29 tests:

| File | Tests | What it checks |
|---|---|---|
| `cursor_tests.rs` | 2 | little-endian fields both ways; reading past the end is `Short`, leftovers are `Length` |
| `addr_tests.rs` | 2 | random address kinds from the top two bits; address types, including the identity types an advertising report can carry |
| `command_tests.rs` | 3 | Reset is `03 0c 00`; LE opcodes are OGF 8; a command round-trips; the parameter length is checked and capped at 255 |
| `event_tests.rs` | 4 | Command Complete for Reset; Command Status; Disconnection Complete; Number of Completed Packets; events whose length disagrees |
| `le_meta_tests.rs` | 3 | LE Connection Complete; LE Long Term Key Request; advertising reports, including a truncated report and data over 31 octets |
| `acl_tests.rs` | 2 | handle and flags in one word; reserved handles, broadcast, and bad lengths refused |
| `l2cap_tests.rs` | 2 | basic frames round-trip; LE fixed channel IDs |
| `signal_tests.rs` | 2 | a signaling command round-trips, identifier 0 and short data refused, Command Reject laid out; connection parameters inside and just outside each range |
| `central_tests.rs` | 2 | a parameter update in range accepted and handed on, one out of range rejected; other requests and malformed updates rejected as not understood; a reject not answered; an unparsable frame an error |
| `att_tests.rs` | 3 | ATT requests byte by byte; responses parsed and bad lengths refused; 16-bit and base-range 128-bit UUIDs, and which opcodes are requests |
| `reassembly_tests.rs` | 4 | fragments join; a header split across fragments; out-of-order, oversized and overlong fragments refused; a start over an unfinished PDU drops it and keeps the new one |

Each expected byte string is laid out by hand from the cited section of the
Core Specification. None is taken from another project's tests.
