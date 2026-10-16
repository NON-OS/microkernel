# capsule_driver_fido

Status: wired, not yet signed, and held to no one. The kernel mirror
(`src/hardware/fido_capsule`) embeds and spawns it under
`nonos-capsule-driver-fido` wherever the inventory sees a USB host, after
the hub walker. It has no owner certificate yet, so no image carries it
until the owner signs it. Its endpoint is held `KERNEL_ONLY`: it finds a
key and says so, and serves nobody until the owner names who may ask a
key to sign (Decision 9).

## Role

`driver.fido0` drives a FIDO2 security key on CTAPHID, on any USB host's
port, root or virtual. One client at a time holds the key and passes it
CTAP2 requests (command byte and CBOR), getting back the key's answers.
Both pass through unread. The key's user says yes by touching it; this
driver never answers for the key or for its user, and never sends a
request of its own but INIT and CANCEL.

```text
client --NFID--> driver.fido0 --CTAPHID reports (interrupt OUT)--> host --> key
                 driver.fido0 <--answer / KEEPALIVE (interrupt IN)--- host
```

The CTAPHID framing, reassembly, channel INIT and the request in flight
are pure (`src/ctaphid`). The report descriptor's application reader is
`driver.usb_hid0`'s and the holder and framing are `driver.cdc_acm0`'s,
mounted by path. This capsule adds the binding (`src/bind.rs`,
`src/up.rs`), the requests (`src/request.rs`, `src/ops.rs`,
`src/session.rs`) and the loop (`src/run.rs`, `src/pump.rs`,
`src/gone.rs`).

## Microkernel contract

- `MkServiceLookup` finds each USB host; every report to the key is an
  IPC call to its host.
- `MkIpcRecvFrom` serves `service:4526:driver.fido0`; `MkIpcReply`
  answers. The kernel delivers nothing to it until the held table names a
  client.
- `MkPidAlive` asks whether the client holding the key is still there.
- `MkMmap` for the heap; `MkDebug` for the `[fido]` lines, when granted.

## Interface contract

NFID version 1: the host wire's 20-byte header with magic `NFID`; a
reply's payload starts with a 4-byte status. Every op but HEALTHCHECK
answers -19 while there is no key, and every op but HEALTHCHECK and OPEN
-1 from a caller that does not hold it.

- `HEALTHCHECK` (1): 0.
- `OPEN` (2): `vendor u16, product u16, caps u8, major u8, minor u8,
  build u8` (CTAPHID capabilities and the key's version). -16 while a
  live client holds the key.
- `CLOSE` (3): a running request is cancelled.
- `CBOR` (4) 1 to 4000 bytes: 0 once sent; -16 while one runs.
- `RESULT` (5): the answer, status byte first, as the key gave it. -11
  with `keepalive u8` while it runs (0 nothing yet, 1 working, 2 waiting
  for a touch); -5 with `code u8` when the key sent CTAPHID_ERROR, -5
  alone for a broken answer; -110 when the key said nothing for 5 s; -90
  for an answer over 4000 bytes; -22 with none asked.
- `CANCEL` (6): CTAPHID_CANCEL; the key ends the request with its own
  status, which RESULT gives.

## Wire format

As above; every field little-endian. To the key, CTAPHID (as recalled,
unverified): 64-byte reports, INIT 86h, CBOR 90h, CANCEL 91h, KEEPALIVE
BBh, ERROR BFh. INIT is taken only with our nonce and a channel neither 0
nor broadcast; a message is at most 7609 bytes, its continuations 0 to 127
in order, another channel's reports never spliced in, and one opened
again before it ended is broken. KEEPALIVE is taken only with one byte,
status 1 or 2; ERROR only with one byte, its code passed up. To the host, the host wire (`NXHC`): CONTROL_TRANSFER,
ALLOC_TRANSFER_RING (0x09), INTERRUPT_IN (0x0E), INTERRUPT_OUT (0x25),
CLAIM_INTERFACE (0x26) and SLOT_OF_PORT (0x27).

## Authority

`CAPSULE_REQUIRED_CAPS = 0x18`: IPC, Memory. `CAPSULE_OPTIONAL_CAPS =
0x100`: Debug, in a serial-debug build only. No Driver, DeviceEnum, Mmio,
Irq, Dma, Pio or Crypto: it reaches the key only through a host, and its
INIT nonce only has to tell its own answer apart. Its endpoint is held to
no one: a key signs for whoever reaches it once touched, so who may ask is
the owner's to decide.

## Current implemented surface

- A HID interface with 64-byte interrupt IN and OUT endpoints whose
  report descriptor opens with usage page F1D0h, usage 1.
- A channel of its own from INIT at bind; CTAP2 keys only (the CBOR
  capability); a CTAP1-only key is not bound.
- One request at a time, keep-alives passed on, CANCEL.
- A composite key (an OTP keyboard beside the FIDO interface) is shared:
  usb_hid0 binds and configures it and claims its keyboard; this driver
  finds the shared slot (OP_SLOT_OF_PORT), never configures it again,
  claims the FIDO interface and drives only that. Either may leave first;
  the host keeps the slot until both have.

## State ownership

The capsule owns the key's slot from bind until it leaves, its channel,
the holder and the request in flight.

## Operating rules

One holder at a time. Nothing is synthesised: no answer, no keep-alive,
no user presence. A request that ends without its answer (its client
gone, the key quiet) is cancelled and costs the channel; the next request
goes on a new one, so the late answer can never pass for its own. An
unread answer is dropped when its holder leaves.

## Privacy and persistence

Requests and answers are held only while in flight, for their holder, and
never logged or written anywhere. The log names the key (vendor and
product) and nothing it says.

## Runtime lifecycle

Spawned once, after the USB hosts and usb_hub0. It looks for a key every
second until one binds. While a request runs it reads the key every 5 ms
between requests; otherwise it waits in the kernel.

## Failure model

A key whose configuration, endpoints or INIT fails is not bound and is
tried again. A key quiet for 5 s mid-request fails it (-110). A holder
the kernel says is gone is let go within a second, its request cancelled.
A key that leaves its port is given back and its holder let go.

## Release target

USB FIDO2 keys for one client the owner names: a browser's WebAuthn, or
an SSH agent with `sk-` keys.

## Release evidence

`fido_proofs` covers the key finding, the reports a request goes out in,
the channel INIT (nonce, broadcast refused, silence), and the request in
flight (keep-alives, answers across reports, errors, broken sequences,
quiet and abandoned requests). The application reader is covered in
`usb_proofs`, interrupt OUT in `xhci_proofs`, the held table in
`kernel_proofs`. No boot yet.

## Release checklist

- Decision 9: who may hold driver.fido0.
- Signed by the owner and added to an image's features.
- A boot with a key; QEMU's `u2f-passthru`/`u2f-emulated` (as recalled,
  unverified) speak CTAP1 only, so they are no test of this.

## Real hardware bring-up checklist

- authenticatorGetInfo (04h) answered with no touch.
- makeCredential: keep-alive 2 until touched, then the answer.
- CANCEL while it waits: the key's own cancel status comes back.

## Explicit non-goals today

No CTAP1/U2F messages (MSG, 83h), no WINK or LOCK, no answers over 4000
bytes, no more than one key, no NFC or BLE. A shared device is read in
its first configuration, the one usb_hid0 sets.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/fido_proofs && cargo test --release)`, with
`usb_proofs`, `xhci_proofs` and `kernel_proofs`.
