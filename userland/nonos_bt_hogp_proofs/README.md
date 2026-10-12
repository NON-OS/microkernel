# nonos_bt_hogp_proofs

Host tests for `nonos_bt_hogp`. The crate depends on `nonos_bt_hogp` and
`nonos_bt_wire` by path and calls their public API. Everything in `src/` is
`#[cfg(test)]`.

`sim/` has two parts:
- A peripheral's ATT server, written from Vol 3, Part F, 3.4. It works
  over an attribute database built from GATT declarations, with the
  default ATT_MTU of 23. It answers discovery with as many entries of one
  length as fit, keeps every write, and can be told to misbehave.
- `Run`, which passes PDUs between the host and the peripheral.

14 tests:

| File | Tests | What it checks |
|---|---|---|
| `discovery_tests.rs` | 2 | a keyboard with a pointer: the right handles found past a vendor 128-bit characteristic and a Report Reference, boot mode by Write Command, both CCCDs by Write Request, reports become events, release on disconnect; a combined device with two HID services |
| `refuse_tests.rs` | 3 | no HID service, or no report that notifies; a report without a CCCD; security errors ask for pairing, other errors end discovery |
| `hostile_tests.rs` | 2 | a peripheral whose answers go back over earlier handles; more HID services than the bound; a descriptor outside its characteristic |
| `bearer_tests.rs` | 3 | requests to the host refused and commands ignored while discovery goes on; indications confirmed and reports before configuration ignored; responses that answer nothing, and malformed ones |
| `timeout_tests.rs` | 1 | 30 s of silence ends the bearer, and nothing is sent after |
| `report_tests.rs` | 3 | keys and modifiers down and up, releases first; rollover and wrong lengths change nothing; mouse motion, three buttons, extra bits and octets ignored |

Seven of the tests were checked against broken code, each break on its
own:
- handles allowed to go back;
- rollover not ignored;
- indications not confirmed;
- no boot-mode write;
- descriptors accepted outside their characteristic;
- the timer off;
- an Error Response for another request accepted.

Each made at least one test fail.
