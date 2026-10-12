# nonos_bt_bond_proofs

Host tests for `nonos_bt_bond`. The crate depends on `nonos_bt_bond`,
`nonos_bt_smp` and `nonos_bt_wire` by path and calls their public API.
Everything in `src/` is `#[cfg(test)]`.

32 tests:

| File | Tests | What it checks |
|---|---|---|
| `record_tests.rs` | 3 | a bond with an IRK laid out octet by octet as documented, and read back; one without an IRK has its flag clear and its field zero; a record that breaks a rule is not written |
| `record_refuse_tests.rs` | 4 | wrong lengths and versions; unknown flags, legacy (no Secure Connections) and reserved octets; fields out of range; an IRK field that disagrees with its flag |
| `pairing_tests.rs` | 3 | the distributed identity kept; without one, a public or static address is the identity; a private address with no identity refused |
| `local_tests.rs` | 2 | this host's IRK round-trips in 17 octets; a zero key, a wrong length and a wrong version refused |
| `book_tests.rs` | 2 | identities found by type and address, the same octets under the other type not; a private address found by the IRK that resolves it, a stranger's and a non-resolvable one not |
| `book_limit_tests.rs` | 2 | a new bond replaces the old one for its identity, and is removed; fifteen bonds held, a sixteenth refused, one renewed while full |
| `memory_keys_tests.rs` | 1 | sixteen records by kind, a seventeenth refused, ids distinct and never zero, a deleted id not found |
| `store_tests.rs` | 4 | bonds read back by a restarted capsule; a renewed bond kept once, a forgotten one gone; an unreadable record counted and left; a keyring that cannot list is an error |
| `store_open_tests.rs` | 1 | sixteen bonds in the keyring: fifteen in the book, one counted, all still kept |
| `store_fault_tests.rs` | 3 | a refused store keeps the old bond; a delete and its rollback both refused still keep the old bond, and the stray is read back once; a full book stores nothing new but renews |
| `store_rollback_tests.rs` | 2 | a refused delete leaves only the old record for a restart to read; an old record already gone lets the renewal stand |
| `local_store_tests.rs` | 3 | this host's IRK drawn once and read back after a restart; a zero draw refused and nothing kept; an unreadable key passed over, a refused store an error |
| `rotation_tests.rs` | 2 | an address due at once and then every 15 minutes, each resolvable under the key, a clock that went back not holding it; a forbidden draw makes nothing and keeps the address |

Breaks tried, each caught:
- records and book: the reserved octets checked short; the IRK flag check
  gone; a static identity looked up as public; the book one bond over its
  limit; the rotation one millisecond late; a private address accepted as
  an identity;
- store: the rollback after a refused delete gone; the full-book check
  on a renewal; the limit one over; an unreadable record or a bond past
  the book not counted; an old record already gone treated as an error;
  a kept IRK not used; a refused IRK store ignored; an old id kept after
  forget;
- memory keys: zero handed out as an id; the limit one over; an id in
  use handed out again.

One change is not observable from the API and so not caught: dropping the
overwrite of a deleted record's bytes in `MemoryKeys`. The bytes are freed
either way; the overwrite keeps a key out of freed memory.
