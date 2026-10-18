# bt_proto

`nonos_bt_proto` is NBTC v1, the wire format of `driver.bt_usb0`'s control
service (`userland/capsule_driver_bt_usb`), written once for both ends:
the capsule answers it, Settings' Bluetooth page asks it. `no_std`, no
dependencies. The table of ops is in the capsule's README and letter E-25.

## Header

`request(out, op, request_id, body)` lays out the 20-byte (`HDR_LEN`)
header, little-endian:

| Bytes | Field |
|---|---|
| 0..4 | `MAGIC`, "NBTC" |
| 4..6 | `VERSION`, 1 |
| 6..8 | op |
| 8..10 | flags, zero in v1 |
| 10..12 | reserved, zero |
| 12..16 | request id |
| 16..20 | payload length |

A reply repeats the header, then a signed status word (`STATUS_LEN`) and
the op's payload from `DATA_AT`. `read_reply(buf, op, request_id)` gives
the status and payload of the reply to that request, and nothing for any
other frame.

## Payloads

| Op | Read with | Written with |
|---|---|---|
| HEALTH | `Health::read` | `Health::write` |
| LINK | `Link::read`, `Link::has` | `Link::write` |
| BATTERY | `Levels::read` | the capsule's `nbtc/look.rs` |
| BONDS | `Bonds::read` | the capsule's `nbtc/bonds.rs` |
| PASSKEY | `Asked::read` | `Asked::write` |
| FORGET, ANSWER, SCAN | the capsule | `forget_body`, `answer_body`, `scan_body` |

Every reader refuses what v1 does not define: a state or flag past the
table, a flag octet other than 0 or 1, a count that does not match the
length or exceeds `BONDS_MAX` or `LEVELS_MAX`, a percent over 100, a
passkey over six digits, or digits where the kind carries none.
`REPLY_MAX` is the longest reply, a full bond list.

## Verification

- `userland/bt_usb_proofs` (`nbtc_proofs/client_*`): what the capsule
  answers reads back with these readers, and the bodies these writers lay
  are what it takes.
- `userland/capsule_settings_proofs` (`bt_tests/proto_*`): each reader
  refuses what is not a v1 reply.
