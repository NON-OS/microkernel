# nonos_bt_smp_proofs

Host tests for `nonos_bt_smp`. The crate depends on `nonos_bt_smp` and
`nonos_bt_wire` by path and calls their public API. Everything in `src/` is
`#[cfg(test)]`.

`sim/` is a responder written from Vol 3, Part H, 2.3.5.6, separately from
the initiator under test. It checks the initiator's commitments and DHKey
check itself, and it can be told to cheat. `Run` passes PDUs between the
two sides one at a time and plays both users as the test asks.

34 tests:

| File | Tests | What it checks |
|---|---|---|
| `cmac_tests.rs` | 2 | RFC 4493's four examples; a message given in parts is one message |
| `funcs_tests.rs` | 5 | f4, f5, f6 and g2 against the sample data of Vol 3, Part H, Appendix D; the f5 address form |
| `ecdh_tests.rs` | 4 | the debug secret gives the debug public key; two pairs agree on a DHKey; off-curve, infinity and debug keys refused; zero and out-of-range secrets refused |
| `method_tests.rs` | 3 | a keyboard types what the host shows; a mouse, or no MITM, is Just Works; the rest of the table |
| `pdu_tests.rs` | 4 | this host's Pairing Request byte by byte; bad pairing parameters refused; exact lengths, unknown codes refused; air order is reversed function order |
| `pairing_tests.rs` | 3 | Just Works with a device that has no IO, with four PDUs and no commitment from us; numeric comparison shows the responder's number; a responder that wants no keys is paired at encryption |
| `pairing_passkey_tests.rs` | 2 | passkey shown here and typed here, each twenty rounds |
| `pairing_fail_tests.rs` | 3 | a confirm its random does not open; a wrong DHKey check; the debug key, an off-curve key and our own key sent back. Each ends with the right reason and no LTK |
| `pairing_user_tests.rs` | 3 | the user sees different numbers; a passkey cancelled or out of range; a mistyped passkey, caught by the responder |
| `pairing_refuse_tests.rs` | 3 | legacy-only, short-key and OOB responses refused before our public key goes out; PDUs out of place, unknown or malformed; identity keys in the wrong order |
| `pairing_timer_tests.rs` | 2 | 30 s of silence ends pairing, silently, and nothing is answered after; calls in the wrong state, a request without Secure Connections, and a broken random source refused |

Every whole pairing checks that both sides derive the same LTK.

Six of the tests were checked against broken code, each break on its own:
- the wrong passkey bit per round;
- one round too few;
- the wrong side's IOcap in Ea;
- no check for our own key sent back;
- identity address accepted before the IRK;
- the SMP timer off.

Each made at least one test fail.
