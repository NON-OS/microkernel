# capsule_driver_ccid

Status: wired, not yet signed, and held to no one. The kernel mirror
(`src/hardware/ccid_capsule`) embeds and spawns it under
`nonos-capsule-driver-ccid` wherever the inventory sees a USB host, after
the hub walker. It has no owner certificate yet, so no image carries it
until the owner signs it. Its endpoint is held `KERNEL_ONLY`: it finds a
reader and says so, and serves nobody until the owner names the client.

## Role

`driver.ccid0` drives a USB CCID smart-card reader of the APDU kind (a
desk reader, a token's built-in reader) on any USB host's port, root or
virtual. One client at a time holds the reader, powers the cards in its
slots and passes them command APDUs, getting back response APDUs. The
APDUs pass through unread: what they say is the card's and the client's.

```text
client --NCCD--> driver.ccid0 --PC_to_RDR (bulk OUT)--> host --> reader --> card
                 driver.ccid0 <--RDR_to_PC (bulk IN)--- host
                 driver.ccid0 <--NotifySlotChange (interrupt IN)-- host
```

The reader finding, the bulk messages, the exchange and the slot memory
are pure (`src/ccid`); the host wire client is `nonos_usbnet`'s, and the
holder and the framing are `driver.cdc_acm0`'s (`owner.rs`, `frame.rs`),
mounted by path. This capsule adds the binding (`src/bind.rs`), the card
requests (`src/card.rs`, `src/ops.rs`, `src/session.rs`) and the loop
(`src/run.rs`, `src/pump.rs`, `src/gone.rs`).

## Microkernel contract

- `MkServiceLookup` finds each USB host; every request to the reader is an
  IPC call to its host.
- `MkIpcRecvFrom` serves `service:4524:driver.ccid0`; `MkIpcReply`
  answers. The kernel delivers nothing to it until the held table names a
  client.
- `MkPidAlive` asks whether the client holding the reader is still there.
- `MkMmap` for the heap; `MkDebug` for the `[ccid]` lines, when granted.

## Interface contract

NCCD version 1: the host wire's 20-byte header with magic `NCCD`; a
reply's payload starts with a 4-byte status. Every op but HEALTHCHECK
answers -19 while there is no reader, and every op but HEALTHCHECK and
OPEN -1 from a caller that does not hold it. A command the reader failed
answers -5 with `bError u8, icc u8` (icc 2: no card); -110 when the reader
said nothing in time.

- `HEALTHCHECK` (1): 0.
- `OPEN` (2): `vendor u16, product u16, max_slot u8, 0, 0, 0, features
  u32`. -16 while a live client holds the reader.
- `CLOSE` (3): every slot the holder powered is powered off.
- `POWER_ON` (4) `slot u8`: the card's ATR.
- `POWER_OFF` (5) `slot u8`: 0.
- `STATUS` (6) `slot u8`: `icc u8, changed u8, 0, 0`; `changed` is 1 when
  the reader said the slot changed since the last STATUS.
- `TRANSMIT` (7) `slot u8, 0, 0, 0`, then one command APDU: the response
  APDU. -22 for one longer than the reader's message, -90 for a response
  longer than 4000 bytes.

## Wire format

As above; every field little-endian. To the reader, CCID 1.1's bulk
messages (as recalled, unverified): PC_to_RDR_IccPowerOn (62h),
IccPowerOff (63h), GetSlotStatus (65h), XfrBlock (6Fh); RDR_to_PC_DataBlock
(80h), SlotStatus (81h), NotifySlotChange (50h). To the host, the host
wire (`NXHC`): CONFIGURE_BULK, CONTROL_TRANSFER, BULK_OUT, BULK_IN_POLL,
RESET_BULK, ALLOC_TRANSFER_RING (0x09) and INTERRUPT_IN (0x0E).

## Authority

`CAPSULE_REQUIRED_CAPS = 0x18`: IPC, Memory. `CAPSULE_OPTIONAL_CAPS =
0x100`: Debug, in a serial-debug build only. No Driver, DeviceEnum, Mmio,
Irq, Dma or Pio: it reaches the reader only through a host. Its endpoint
is held to no one: a card signs, decrypts and opens doors for whoever
reaches its reader, so who may is the owner's to decide.

## Current implemented surface

- One reader: an interface of class 0Bh with a bulk pair and its CCID
  class descriptor, whose dwFeatures say short or extended APDU exchange.
- Every slot it has, one command at a time, each under its own sequence
  number; an answer to another is let go.
- Time extensions from the card, up to 64 in a row, each waited 3 s.
- Slot changes from the interrupt IN endpoint where it has one.

## State ownership

The capsule owns the reader's slot from bind until it leaves, the holder,
the sequence number, and which slots the holder powered or the reader
said changed.

## Operating rules

One holder at a time. A card is never handed from one holder to the next
in the state the first left it (a PIN verified, an application chosen):
every slot a holder powered is powered off when it closes, when the kernel
says it is gone, or when another opens after it died. APDUs are not read,
kept or logged.

## Privacy and persistence

Nothing a card says is kept past the reply it rides in, and nothing is
written anywhere. The log names the reader (vendor and product) and never
an ATR or an APDU.

## Runtime lifecycle

Spawned once, after the USB hosts and usb_hub0. It looks for a reader every
second until one binds. While the reader is held it looks at slot changes
every 20 ms between requests; otherwise it waits in the kernel.

## Failure model

A reader whose bulk pipes or configuration is refused is not bound and is
tried again. A TPDU or character reader is named once and left. A refused
slot-change endpoint leaves the reader working without `changed`. A halted
pipe is cleared and the command answered -5. A holder the kernel says is
gone is let go within a second, its cards powered off. A reader that
leaves its port is given back and its holder let go.

## Release target

Desk readers and readers built into tokens, for one client the owner
names: an OpenPGP or PIV card agent, or a national ID client.

## Release evidence

`ccid_proofs` covers the reader finding, the messages, the exchange (whole
answers across transfers, late answers, time extensions and their bound,
time running out, halted pipes) and the slot memory. The held-table proofs
in `kernel_proofs`. No boot yet.

## Release checklist

- The owner names the client the endpoint is held to.
- Signed by the owner and added to an image's features.
- A boot with a reader. QEMU's `usb-ccid` (as recalled, unverified) is one
  to try first.

## Real hardware bring-up checklist

- A desk reader: an ATR read, SELECT of a PIV applet answered 90 00.
- A card pulled and another put in between two STATUS: `changed` is 1.
- The holder killed with a card powered: the card is powered off.

## Explicit non-goals today

No TPDU or character readers (T=0 and T=1 done on the host), no PIN pad
(Secure, 69h), no escape commands, no abort (control request ABORT), no
clock or data-rate setting, no more than one command in flight, no more
than one reader, and no APDU over one message (4096 bytes at most).

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/ccid_proofs && cargo test --release)` and
`(cd userland/kernel_proofs && cargo test --release)`.
