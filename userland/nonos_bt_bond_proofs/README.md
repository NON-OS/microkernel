# nonos_bt_bond_proofs

Host tests for `nonos_bt_bond`. The crate depends on `nonos_bt_bond`,
`nonos_bt_smp` and `nonos_bt_wire` by path and calls their public API.
Everything in `src/` is `#[cfg(test)]`.

18 tests:

| File | Tests | What it checks |
|---|---|---|
| `record_tests.rs` | 3 | a bond with an IRK laid out octet by octet as documented, and read back; one without an IRK has its flag clear and its field zero; a record that breaks a rule is not written |
| `record_refuse_tests.rs` | 4 | wrong lengths and versions; unknown flags, legacy (no Secure Connections) and reserved octets; fields out of range; an IRK field that disagrees with its flag |
| `pairing_tests.rs` | 3 | the distributed identity kept; without one, a public or static address is the identity; a private address with no identity refused |
| `local_tests.rs` | 2 | this host's IRK round-trips in 17 octets; a zero key, a wrong length and a wrong version refused |
| `book_tests.rs` | 2 | identities found by type and address, the same octets under the other type not; a private address found by the IRK that resolves it, a stranger's and a non-resolvable one not |
| `book_limit_tests.rs` | 2 | a new bond replaces the old one for its identity, and is removed; fifteen bonds held, a sixteenth refused, one renewed while full |
| `rotation_tests.rs` | 2 | an address due at once and then every 15 minutes, each resolvable under the key, a clock that went back not holding it; a forbidden draw makes nothing and keeps the address |

Six breaks were tried, each caught: the reserved octets checked short; the
IRK flag check gone; a static identity looked up as public; the book one
bond over its limit; the rotation one millisecond late; a private address
accepted as an identity.
