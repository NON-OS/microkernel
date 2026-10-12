# capsule_driver_bt_usb

Status: not in the image. The capsule builds and its host proofs pass,
needs B-43 applied to `capsule_driver_xhci` first (below).

## Role

`capsule_driver_bt_usb` is a Bluetooth LE host for USB controllers. It
finds a controller on driver.xhci0's ports, brings it up with standard HCI
commands only, finds a HID keyboard or mouse with a passive scan, pairs
with LE Secure Connections, and posts the device's input exactly as
driver.usb_hid0 posts a USB keyboard's and mouse's.

```text
Bluetooth keyboard or mouse
        |  LE, encrypted after pairing
        v
USB Bluetooth controller (class E0/01/01)
        |
        v
driver.xhci0 -- control OUT, interrupt IN (events), bulk pair (ACL)
        |
        v
driver.bt_usb0 -- HCI, L2CAP, SMP, ATT/GATT, HOGP --> input ring
```

## What it needs from driver.xhci0 (B-43)

B-43 has to be applied before this capsule can bind anything:
- one slot holds the event endpoint (interrupt IN) and the ACL bulk pair
  together;
- an interrupt read is up to wMaxPacketSize bits 10:0, capped at 64
  octets, and a longer read is `E_INVAL`;
- `OP_BULK_IN_POLL` (0x0014) is dispatched.

Without it, the second endpoint configured is refused, and the capsule
logs `controller not usable: ACL endpoints refused`.

## The layers

| Module | What it does |
|---|---|
| `usb` | Chooses the configuration with the Bluetooth interface (`choose`), arms each event read with the whole packet and never more than 64 octets (`limit`), and gives driver.xhci0 the three endpoints before SET_CONFIGURATION (`bind`). The interrupt operations are 0x0009 and 0x000E (`ops`). The client call is `nonos_usbnet`'s own file, compiled here. |
| `link` | One link, from scan to end, over `nonos_bt_hci`, `nonos_bt_smp` and `nonos_bt_hogp`. It holds no libc call, so the proofs run all of it. |
| `hid` | Input posting. `capsule_driver_usb_hid`'s own keymap, key event, repeat and posting files are compiled here, so a Bluetooth key is posted with the same code, flags, layout and repeat as a USB one. |
| `ring` | The bond store's `Keys` over the keyring: OP_STORE, OP_RETRIEVE, OP_DELETE and OP_LIST (L33), with the layouts `capsule_keyring` reads. Its request and answer layouts are pure, so the proofs run them. |
| `run` | The clock, log, random source and waiting; the bond store opened; finding the controller on the ports, and the loop of links. |

## A link

1. Between scans, a new resolvable private address if the last one has
   served 15 minutes (Vol 3, Part C, Appendix A), given to the controller
   with LE Set Random Address.
2. Passive scan, for at most 10 s. The first connectable advertiser that
   names the HID service (0x1812) or a HID appearance is taken. The scan
   is stopped whatever happened.
3. LE Create Connection, from the private address.
4. A bonded peer, found by its identity or by the IRK that resolves its
   address, is encrypted with the kept key and not paired again. If it
   answers PIN or Key Missing it has dropped the bond: ours is forgotten
   and the next link pairs afresh.
5. Any other peer: SMP pairing at once, LE Secure Connections only. This
   host offers its IRK and public identity, so the peer can resolve its
   later addresses. A keyboard with no
   display pairs by passkey entry: the capsule logs six digits for the
   user to type. A mouse pairs by Just Works. A peer that wants a number
   compared or digits typed here is declined; the capsule has no way yet
   to ask the user.
6. LE Enable Encryption with the derived key, then the identity keys:
   the peer's, then this host's. The bond is kept.
7. HID over GATT, Boot Protocol; input to the input ring. Keys are posted
   only from a bond the user's passkey protected: Just Works needs no one,
   so a device in range that pairs that way and calls itself a keyboard
   could otherwise type into the machine. Its pointer motion and buttons
   are posted, since a mouse can only pair by Just Works.
8. Requests from the peer are answered meanwhile: its reads of this host's
   GAP database, and connection timing it asks for (applied with LE
   Connection Update when it is in range).

A link that ends from the peer's side goes straight back to step 1. One
this host gave up on (pairing declined, encryption refused, a 30 s
pairing or ATT timer, a hardware error) is ended with Disconnect, and the
next look waits 30 s.

## Not yet

- Bonds last for this boot only (Decision 27). They are kept in the
  keyring when it keeps this capsule's records (L33), else in this
  capsule's memory until it stops; the log says which.
- No allow-list beyond the bonds: an unbonded HID advertiser is still
  paired, and its keys post only after a typed passkey.
- No service: the capsule answers no requests on driver.bt_usb0 yet.

## Log

`bt-usb:` lines: `bonds last for this boot only (Decision 27)`, where
bonds are kept, `started`, `controller up`, `connected`, the passkey to
type, `paired`, `bond kept`, `encrypted with the kept bond`, `HID ready: keyboard mouse`, `link lost` or `link ended:`
with the reason, and `controller given back:` with the reason.

## Contract

Caps `0x10200038` (IPC, Memory, Crypto, Debug, Keyring). Crypto is what
the CryptoRandom syscall is gated on; pairing draws its key pair and
nonces there, and the capsule stops if a draw fails. Keyring is for its
bonds and its own IRK. Service `driver.bt_usb0` on
port 4342, reply port 4343 (`endpoint.4294967368`), from track E's range.
Proofs: `userland/bt_usb_proofs`.

## Microkernel contract

The manifest grants `IPC`, `Memory`, `Crypto`, `Debug` and `Keyring`:

```text
CAPSULE_REQUIRED_CAPS = 0x10200038
```

The capsule finds driver.xhci0 and the keyring with `MkServiceLookup`, learns
`MkIpcCall` (bounded by a timeout). It waits with `MkIpcRecvFrom` and a
timeout on its own inbox, which serves no requests. It posts input with
`MkInputEventPost`, draws pairing's keys and nonces, its IRK and its
addresses with `CryptoRandom`, reads the clock with `MkUptimeMs`, and
writes its `bt-usb:` lines with `MkDebug`. It makes no enumeration, MMIO, IRQ, DMA or PIO broker syscall:
driver.xhci0 owns the controller.

## Interface contract

No requests are served on driver.bt_usb0 yet. Its outputs are input
events, the same kinds and codes driver.usb_hid0 posts:

| Event | From |
|---|---|
| `INPUT_KIND_KEY_DOWN`, `INPUT_KIND_KEY_UP` | boot keyboard reports, through the USB HID driver's keymap, layout and repeat |
| `INPUT_KIND_BUTTON_DOWN`, `INPUT_KIND_BUTTON_UP` | boot mouse buttons 1 to 3 |
| `INPUT_KIND_POINTER_REL` | boot mouse motion |

Toward driver.xhci0 it uses the operations of `nonos_usbnet` (slots,
descriptors, control, bulk) and two of its own: 0x0009 to add the event
endpoint and 0x000E to read it.

## Authority

```text
allowed:   HCI over USB, L2CAP fixed channels, SMP pairing, ATT/GATT client, HOGP boot reports, input events
forbidden: xHCI ownership, USB scheduling, DMA buffers, vendor firmware, BR/EDR, keys anywhere but this process and its own keyring records
```

## Privacy and persistence

The scan is passive: nothing of this host goes on the air until it
connects to a HID device. Pairing's private key is drawn from the kernel
and overwritten when dropped (`nonos_p256`). A bond (LTK, the peer's IRK
and identity address) and this host's IRK are kept as keyring records
of types 10 and 11, owned by driver.bt_usb0, or in this capsule's memory;
either way only for this boot (Decision 27). Every copy read back is
overwritten once decoded. The log carries no address, no key and no
input; it does carry the passkey to type while pairing, which is what the
user needs and is worthless once pairing ends. This host scans and
connects from a resolvable private address renewed every 15 minutes;
only bonded peers, given its IRK, can tell two of them apart.

## Runtime lifecycle

Started wherever an xHCI controller is. Every second the free root ports
are looked at; a controller is bound, brought up, and runs links one after
another (scan, connect, pair, serve) until a USB transfer fails or the
controller stops answering. Its slot is then given back and the ports are
looked at again; a port that fails three binds is left until it reconnects.

## Failure model

Every wait is bounded: a command by the HCI poll budget, a scan by 10 s,
pairing and ATT by their 30 s timers. Data from the peer that does not
parse costs that PDU only. A refused pairing, a refused key, a silent
peripheral or a hardware error ends the link with Disconnect and a 30 s
pause. A failed USB transfer gives the controller back. A failed random
draw stops the capsule rather than pair with guessable keys.

## Current implemented surface

The layers and the link above: one controller, one HID device at a time,
Boot Protocol keyboard and mouse.

## Wire format

HCI over USB as Vol 4, Part B, 2 has it: commands as class request 0x20/0
on the default endpoint, events from the interrupt endpoint, ACL data on
the bulk pair, no packet indicator. Toward driver.xhci0, each request
carries its 20-byte header and each reply adds a 4-byte signed status:

```text
magic_le32, version_le16, op_le16, flags_le16, 0_le16, request_id_le32, len_le32
```

## State ownership

`driver.bt_usb0` owns the bound controller's three endpoints, the HCI
host's command and buffer state, and one link's session: its pairing state,
its keys, its HID host and the keys held down, and the bonds and this
host's IRK. driver.xhci0 owns slots,
endpoint contexts, rings and DMA.

## Operating rules

- Keep USB transfer mechanics in driver.xhci0.
- Send no vendor command; a controller that needs one is refused.
- Pair with LE Secure Connections only; never fall back to legacy pairing.
- Read the HID service only on an encrypted link.
- Bound every length the controller or the peer sends before using it.

## Release target

```text
Bluetooth keyboard -> USB controller -> driver.xhci0 -> driver.bt_usb0 -> input ring
```

Typing in an application with a Bluetooth keyboard, after pairing once.

## Release evidence

Build and host-proof evidence only so far (bt_usb_proofs, 24 tests, with
the HCI, SMP and HOGP proofs underneath). Runtime evidence needs B-43 in
driver.xhci0, the kernel spawn, and a real controller and keyboard: none
is on main yet.

## Release checklist

- B-43 applied to capsule_driver_xhci.
- Capsule builds with zero warnings; clippy clean.
- bt_usb_proofs, nonos_bt_hci_proofs, nonos_bt_smp_proofs and
  nonos_bt_hogp_proofs pass.
- The kernel spawns the capsule.
- On hardware: `bt-usb: controller up`, `paired`, `HID ready: keyboard`,
  then keys in an application.

## Explicit non-goals today

BR/EDR (Classic) devices, more than one device at a time, Report Protocol
(media keys, wheels, absolute pointers), vendor firmware loading, a pairing
dialog, kept bonds and LE privacy (next unit).

## Verification

`cd userland/bt_usb_proofs && RUST_TEST_THREADS=1 cargo test --release`
(24 pass at the time of writing); the capsule's `cargo build --release`
and `cargo clippy` for x86_64-nonos-user. Not verified under QEMU or on
hardware.
