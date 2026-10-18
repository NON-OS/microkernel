# bt_usb_proofs

Host tests for `capsule_driver_bt_usb`. The capsule's own `link`, `usb`
choice and limit, and `hid` modifier files are compiled here through
`#[path]`, unchanged. Everything in `src/` is `#[cfg(test)]`.

`air/` is the far side of the USB pipes the capsule drives:
- a controller that answers bring-up through the HCI proofs' simulator,
  and scan, connect, encryption, Disconnect and LE Connection Update as
  Vol 4, Part E describes them. It cuts events into 16-octet transfers
  and fails the proof on a transfer longer than what was armed;
- the SMP proofs' responder, written apart from the initiator under test;
- the HOGP proofs' peripheral and its databases;
- ACL both ways in 27-octet packets, each freed with Number of Completed
  Packets; it fails the proof if the host asks over ATT before the link is
  encrypted.

Once the host has had nothing to do for a while, scripted acts run:
notifications, signaling frames, raw events, and the link dropping. The
environment's clock moves a millisecond each time it is read, and the
peer's user types whatever passkey the capsule shows.

24 tests:

| File | Tests | What it checks |
|---|---|---|
| `usb_tests.rs` | 3 | an event read arms the whole packet and never more than 64 octets; the Bluetooth configuration and its value taken; another device, an empty list, an over-long event packet, a short header and non-configuration descriptors told apart |
| `pick_tests.rs` | 4 | the HID service in either UUID list; a HID appearance, and others not; report types a central may not answer passed over; broken advertising data ends the walk without a match |
| `modifier_tests.rs` | 2 | the eight modifier usages to their boot report bits; the layout chord |
| `find_tests.rs` | 2 | no HID device: the look ends after 10 s, passive, with the scan stopped; the scan stops before connecting |
| `link_tests.rs` | 1 | a keyboard heard, connected, paired by passkey entry, encrypted with the responder's key, HID ready, a key down and up posted, the link lost |
| `mouse_tests.rs` | 2 | a mouse pairs by Just Works with nothing shown and its motion and button posted and released; a key held when the link drops is released |
| `refuse_tests.rs` | 4 | Numeric Comparison declined and no key used; a refused key ends the link and Disconnect closes it; a silent peripheral runs out the ATT timer; a hardware error ends the link |
| `signal_tests.rs` | 3 | timing in range accepted and applied with LE Connection Update; out of range rejected and not applied; an LE credit channel request rejected |
| `credit_tests.rs` | 1 | three LE buffers freed late: the public key waits whole for buffers, and pairing finishes |
| `early_tests.rs` | 1 | the peer reads the Device Name before pairing and is answered on the clear link |
| `trust_tests.rs` | 1 | a keyboard paired by Just Works moves the pointer but its keys are not posted, and the log says so |

Breaks tried, each caught by at least one test: the 64-octet cap, the
configuration header length, the connectable report types, a zero-length
advertising structure, the scan left on, an accepted update not applied,
Numeric Comparison accepted, a refused key ignored, held input not
released, the modifier bits, the Disconnect reason, a PDU dropped when no
buffer is free, the peer's early reads not answered, and keys posted from
a Just Works bond.

```
RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true
```
