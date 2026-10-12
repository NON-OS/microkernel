# capsule_driver_bt_usb

Status: not in the image. The capsule builds and its host proofs pass,
needs B-43 applied to `capsule_driver_xhci` first (below).

## Role

`capsule_driver_bt_usb` is a Bluetooth LE host for USB controllers. It
finds a controller on driver.xhci0's ports, patches it first if it is a
Realtek chip that needs it (below), brings it up with standard HCI
commands, finds a HID keyboard or mouse with a passive scan, pairs
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
| `nbtc` | NBTC v1, the protocol driver.bt_usb0 answers (below): the frame, who may ask what, and the board it reads and changes. Pure, so the proofs run all of it. |
| `rtl` | A Realtek controller's setup: its ROM matched, its patch and config read and checked against pinned SHA-256s, the image downloaded in fragments. Pure, so the proofs run it against a model ROM. |
| `run` | The clock, log and random source; the vendor files built in (feature `realtek`); the desk, whose every wait answers NBTC; the bond store opened; finding the controller on the ports, and the loop of links. |

## Realtek controllers

Before bring-up resets anything, the capsule asks Read Local Version
Information (Vol 4, Part E, 7.4.1). A controller whose company is
Realtek's (0x005D) and whose HCI version, HCI revision and LMP subversion
are one of these ROMs is patched:

| Chip | ROM answer (LMP subversion, HCI revision, HCI version) | Patch | Config |
|---|---|---|---|
| RTL8723B | 8723, 000B, 06 | rtl8723b_fw.bin | rtl8723bs_config.bin |
| RTL8723D | 8723, 000D, 08 | rtl8723d_fw.bin | rtl8821c_config.bin (needed) |
| RTL8821A | 8821, 000A, 06 | rtl8821a_fw.bin | rtl8821c_config.bin |
| RTL8821C | 8821, 000C, 08 | rtl8821c_fw.bin | rtl8821c_config.bin |
| RTL8761A | 8761, 000A, 06 | rtl8761a_fw.bin | rtl8761bu_config.bin |
| RTL8761BU | 8761, 000B, 0A | rtl8761bu_fw.bin | rtl8761bu_config.bin |
| RTL8822B | 8822, 000B, 07 | rtl8822b_fw.bin | rtl8822b_config.bin (needed) |
| RTL8822CU | 8822, 000C, 0A | rtl8822cu_fw.bin | none |
| RTL8852AU | 8852, 000A, 0B | rtl8852au_fw.bin | none |
| RTL8852BU | 8852, 000B, 0B | rtl8852bu_fw.bin | none |
| RTL8852CU | 8852, 000C, 0C | rtl8852cu_fw_v2.bin | none |
| RTL8851BU | 8851, 000B, 0C | rtl8851bu_fw.bin | none |
| RTL8852BTU | 8852, 0087, 0C | rtl8852btu_fw.bin | none |

The first seven files are in the first layout ("Realtech"), the last six
in the second ("RTBTCore").

1. Read ROM Version (vendor command 0xFC6D) gives one octet, the ROM
   version.
2. For a second-layout chip, its key id: the first octet of a register
   read with vendor command 0xFC61.
3. The patch file and the config are the capsule's own embedded copies.
   Each must have exactly the length and SHA-256
   pinned in `rtl/pins.rs`, `rtl/pins_config.rs` and `rtl/pins_rtbt.rs`,
   or it is refused and nothing is sent. The patch file must also name
   the chip's project in its extension block.
4. The image. From a first-layout file: the patch whose chip id is the
   ROM version plus one, the file's firmware version in its last four
   octets, then the config. From a second-layout file: every subsection
   whose eco is the ROM version plus one, ordered by priority (equal
   priorities later first), security subsections only when the chip has
   a key and only those signed for it.
5. The image goes in Download commands (0xFC20) of 252 octets at most,
   each with an index octet that counts 0 to 7F and then from 1, bit 7
   set on the last.
6. Read Local Version again: a running patch answers another LMP
   subversion. A controller still answering as its ROM is given back.

A patch survives Reset, so the standard bring-up follows as on any
controller. A controller already patched (a capsule restart) answers
another subversion, matches no ROM, and is not patched again. Any other
company's controller is asked nothing more.

The files are built into the capsule with `include_bytes!`, so its seal
covers them, and only with the cargo feature `realtek`, which is off. They
are linux-firmware's `rtl_bt/` files at tag 20260916 (commit
ab23307cfe7f9366c819025ca3e4778299bc2db2), under
`LICENSES/LICENCE.rtlwifi_firmware.txt`: redistribution in binary form,
the seventeen files to `nonos-bootloader/firmware/rtl_bt/` (Decision 19),
then builds with `--features realtek`. `build.rs` refuses the build if a
file is missing or is not its pin, or if `run/files.rs` embeds a file the
pins do not name; the pins are checked again before each download.
Without the feature, a Realtek controller is brought up on its ROM.

## A link

1. Between scans, a new resolvable private address if the last one has
   served 15 minutes (Vol 3, Part C, Appendix A), given to the controller
   with LE Set Random Address.
2. Passive scan, while scanning is on (NBTC's SCAN), for at most 10 s.
   A bonded peer is taken the moment it is heard, from directed
   advertising too, which carries no data to name the HID service by
   (Vol 6, Part B, 2.3.1.2). A device with no bond is taken only while
   the pairing window Settings opens (NBTC's PAIR) is open: the first
   connectable advertiser that names the HID service (0x1812) or a HID
   appearance, at once while no bond is kept, else after waiting 2 s from
   when it was first heard, so a bonded keyboard waking beside a
   stranger's is not passed over for it. The scan is stopped whatever
   happened.
3. LE Create Connection, from the private address.
4. A bonded peer, found by its identity or by the IRK that resolves its
   address, is encrypted with the kept key and not paired again. If it
   answers PIN or Key Missing it has dropped the bond: ours is forgotten
   and the next link pairs afresh.
5. Any other peer: SMP pairing at once, LE Secure Connections only. This
   host offers its IRK and public identity, so the peer can resolve its
   later addresses. A keyboard with no
   display pairs by passkey entry: the capsule logs six digits for the
   user to type and gives them to Settings. A mouse pairs by Just Works.
   A peer that shows digits to type here, or a number to confirm, waits
   for the user's answer from Settings, as long as SMP's 30 s timer
   allows. One pairing closes the window.
6. LE Enable Encryption with the derived key, then the identity keys:
   the peer's, then this host's. The bond is kept.
7. HID over GATT, Boot Protocol; input to the input ring. Keys are posted
   only from a bond the user's passkey protected: Just Works needs no one,
   so a device in range that pairs that way and calls itself a keyboard
   could otherwise type into the machine. Its pointer motion and buttons
   are posted, since a mouse can only pair by Just Works.
8. Then the Battery Service: each Battery Level is read, and followed by
   notification where the device offers it. Each level is logged. A level
   is not input, so a Just Works link gives it too. A battery the capsule
   cannot read ends nothing.
9. Requests from the peer are answered meanwhile: its reads of this host's
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
- Turning scanning off takes effect when the look under way ends, at
  most 10 s later.
- Directed advertising is heard only when the controller takes it as
  addressed to this device (Vol 4, Part E, 7.8.10, filter policy 0x00).
  A peer that directs it to an older private address of this host is not
  heard, and reconnects only once it advertises undirected.

## Log

`Realtek RTL8821C patched with rtl8821c_fw.bin`, or `Realtek setup: `
and the file and reason when it was not (not built in, not the pinned
file, no patch for this ROM, refused, not running).

`bt-usb:` lines: `bonds last for this boot only (Decision 27)`, where
bonds are kept, `started`, `controller up`, `connected`, the passkey to
type, `paired`, `bond kept`, `encrypted with the kept bond`, `HID ready: keyboard mouse`,
`battery 1: 90%` (instances count from 1), `pairing waits on the user, in
Settings`, `link lost` or `link ended:`
with the reason, and `controller given back:` with the reason.

## NBTC v1

driver.bt_usb0's own protocol, on port 4342 with replies on 4343. Its
numbers, header and payload layouts are `nonos_bt_proto`
(`userland/bt_proto`), which Settings reads too. The
frame is NNET's: a 20-byte header (`magic_le32` "NBTC", `version_le16` 1,
`op_le16`, `flags_le16`, `0_le16`, `request_id_le32`, `len_le32`), then the
payload; a reply carries the same header, a signed status word and its
payload, both counted in `len`. Status: 0, or -2 no such bond, -5 the
store refused, -13 refused to this caller, -16 busy (ask again), -22
malformed.

| Op | Name | Who | Request | Reply |
|---|---|---|---|---|
| 1 | HEALTH | Settings, shell | none | version (2), controller up (1) |
| 2 | LINK | Settings, shell | none | state, flags, bond place (0xFF none) |
| 3 | BATTERY | Settings, shell | none | count, then place, instance, percent each |
| 4 | BONDS | Settings | none | count, then identity type, address (6), authenticated, linked each |
| 5 | FORGET | Settings | identity type, address (6) | none; its link ends |
| 6 | PAIR | Settings | none | none; one device may pair for 60 s |
| 7 | PASSKEY | Settings | none | kind (0 none, 1 show, 2 enter, 3 confirm), digits (4) |
| 8 | ANSWER | Settings | kind, digits (4) | none |
| 9 | SCAN | Settings | 1 on, 0 off | none |

LINK's state: 0 no controller, 1 idle, 2 scanning, 3 connecting, 4
securing, 5 ready. Its flags, bits 0 to 5: keyboard, mouse, keys post,
scanning on, pairing window open, pairing waiting on the user. A place is
a bond's index in BONDS's list. ANSWER answers what PASSKEY says is
asked: ENTER with the digits typed, CONFIRM with 1 for the same and 0
for not, NONE to decline; a passkey shown needs no answer.

The kernel holds driver.bt_usb0 to Settings (`app.settings`, `.1`, `.2`)
and `desktop_shell`; the capsule checks each op again, by the sender's
pid against the pid holding each name now, and refuses by default. No
reply carries a key. Identity addresses go to Settings only: the shell
sees places in the bond list, never addresses.

## Contract

Caps `0x10200038` (IPC, Memory, Crypto, Debug, Keyring).
Crypto is what the CryptoRandom syscall is gated on; pairing draws its key
pair and nonces there, and the capsule stops if a draw fails. It has no
filesystem authority: the Realtek files are built in. Keyring is for its bonds and its own IRK. Service `driver.bt_usb0` on
port 4342, reply port 4343 (`endpoint.4294967368`), from track E's range;
it answers NBTC v1 (above).
Proofs: `userland/bt_usb_proofs`.

## Microkernel contract

The manifest grants `IPC`, `Memory`, `Crypto`, `Debug` and `Keyring`:

```text
CAPSULE_REQUIRED_CAPS = 0x10200038
```

The capsule finds driver.xhci0 and the keyring with `MkServiceLookup`, and
calls both with `MkIpcCall` (bounded by a timeout). It waits with `MkIpcRecvFrom` and a
timeout on its own inbox, answers each NBTC request there with
`MkIpcReply`, and looks up who holds `app.settings` and `desktop_shell`
with `MkServiceLookup` for each. It posts input with
`MkInputEventPost`, draws pairing's keys and nonces, its IRK and its
addresses with `CryptoRandom`, reads the clock with `MkUptimeMs`, and
writes its `bt-usb:` lines with `MkDebug`. It makes no enumeration, MMIO, IRQ, DMA or PIO broker syscall:
driver.xhci0 owns the controller.

## Interface contract

Requests are NBTC v1 (above). Its other outputs are input events, the
same kinds and codes driver.usb_hid0 posts:

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
allowed:   HCI over USB, a Realtek patch download from pinned files built in (feature `realtek`), L2CAP fixed channels, SMP pairing, ATT/GATT client, HOGP boot reports, input events, NBTC to Settings and the shell
forbidden: xHCI ownership, USB scheduling, DMA buffers, firmware not pinned, filesystem access, BR/EDR, keys anywhere but this process and its own keyring records
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
pause. A failed USB transfer gives the controller back. So does a Realtek
controller whose files are not their pins; its port is tried again, and
every port is looked at afresh once a minute. A failed random
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
- Send no vendor command but Realtek's setup, and send no file that is
  not exactly its pin.
- Pair with LE Secure Connections only; never fall back to legacy pairing.
- Read the HID service only on an encrypted link.
- Bound every length the controller or the peer sends before using it.

## Release target

```text
Bluetooth keyboard -> USB controller -> driver.xhci0 -> driver.bt_usb0 -> input ring
```

Typing in an application with a Bluetooth keyboard, after pairing once.

## Release evidence

Build and host-proof evidence only so far (bt_usb_proofs, 137 tests, with
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
(media keys, wheels, absolute pointers), vendor setup other than
Realtek's (Intel, MediaTek, Broadcom), and
bonds kept past this boot. The pairing dialog is Settings', over NBTC.

## Verification

`cd userland/bt_usb_proofs && RUST_TEST_THREADS=1 cargo test --release`
(137 pass at the time of writing); the capsule's `cargo build --release`
and `cargo clippy` for x86_64-nonos-user. Not verified under QEMU or on
hardware.
