# capsule_driver_cdc_acm

Status: wired, not yet signed. The kernel mirror
(`src/hardware/cdc_acm_capsule`) embeds and spawns it under
`nonos-capsule-driver-cdc-acm` wherever the inventory sees a USB host,
after the hub walker. It has no owner certificate yet, so no image carries
it until the owner signs it.

## Role

`driver.cdc_acm0` drives a USB CDC Abstract Control Model function: a USB
serial adapter that speaks the class (microcontroller boards, consoles), or
a modem's AT command port. It finds the function on any USB host's port,
root or virtual (behind a hub), and serves one client at a time the bytes
of the line, its settings, DTR and RTS, and what the device says of DCD,
DSR and line errors.

```text
app.terminal --NACM--> driver.cdc_acm0 --bulk IN/OUT, control--> host --> device
                       driver.cdc_acm0 <--SERIAL_STATE (interrupt IN)-- host
```

The function finding, line coding, notifications, the holder and the read
buffer are pure (`src/acm`); the host wire client is `nonos_usbnet`'s. This
capsule adds the notification endpoint's client (`src/notify.rs`), the
binding (`src/bind.rs`), the requests (`src/ops.rs`, `src/control.rs`,
`src/session.rs`) and the loop (`src/run.rs`, `src/pump.rs`).

## Microkernel contract

- `MkServiceLookup` finds each USB host; every request to the device is an
  IPC call to its host.
- `MkIpcRecvFrom` serves `service:4522:driver.cdc_acm0`; `MkIpcReply`
  answers.
- `MkPidAlive` asks whether the client holding the line is still there.
- `MkMmap` for the heap; `MkDebug` for the `[cdc-acm]` lines, when granted.

## Interface contract

NACM version 1: the host wire's 20-byte header with magic `NACM`; a reply's
payload starts with a 4-byte status. Every op but HEALTHCHECK answers -19
while there is no line, and every op but HEALTHCHECK and OPEN -1 from a
caller that does not hold it.

- `HEALTHCHECK` (1): 0.
- `OPEN` (2): `vendor u16, product u16, protocol u8, capabilities u8, 0,
  0`. DTR and RTS are raised. -16 while a live client holds the line.
- `CLOSE` (3): DTR and RTS dropped, unread bytes forgotten.
- `WRITE` (4) 1 to 4000 bytes: `taken u32`.
- `READ` (5) `most u16`: up to that many bytes, none while none came.
- `SET_LINE` (6) `baud u32, stop u8, parity u8, data u8`: -22 for a line
  that is not one, or a device without line coding.
- `SET_CONTROL` (7) `lines u16`: bit 0 DTR, bit 1 RTS.
- `STATUS` (8): `state u16, 0, 0, queued u32`. State bits: 0 DCD, 1 DSR,
  2 break, 3 ring, 4 framing, 5 parity, 6 overrun; the last five are
  events since the previous STATUS.
- `BREAK` (9) `ms u16`: -22 on a device without SEND_BREAK.

## Wire format

As above; every field little-endian. To the host it speaks the host wire
(`NXHC`): CONFIGURE_BULK, CONTROL_TRANSFER, BULK_OUT, BULK_IN_POLL,
RESET_BULK, ALLOC_TRANSFER_RING (0x09) and INTERRUPT_IN (0x0E).

## Authority

`CAPSULE_REQUIRED_CAPS = 0x18`: IPC, Memory. `CAPSULE_OPTIONAL_CAPS =
0x100`: Debug, in a serial-debug build only. No Driver, DeviceEnum, Mmio,
Irq, Dma or Pio: it reaches the device only through a host. Its endpoint
is held to `app.terminal` alone: a modem dials out, a network path outside
the net stack's rules, so anything more is the owner's to decide.

## Current implemented surface

- One ACM function: a communications interface (class 2, subclass 2) whose
  Union names a data interface with a bulk pair. The ACM functional
  descriptor's capabilities decide which requests are sent.
- 115200 8N1 set at bind where the device takes a line coding.
- SERIAL_STATE from the interrupt IN endpoint where it has one.

## State ownership

The capsule owns the device's slot from bind until it leaves, the holder,
the read buffer (16 KiB) and the last line state.

## Operating rules

One holder at a time. Nothing is read from the device while no one holds
the line, and no read is armed that would not fit whole, so a slow client
leaves bytes in the device (which holds them off) rather than losing them
here. DTR is down until OPEN, so a modem does not answer before someone
listens, and dropped at CLOSE, so it hangs up.

## Privacy and persistence

What was read is kept only for the holder, at most 16 KiB, and forgotten
at OPEN, CLOSE, a holder gone or the device gone. Nothing is written
anywhere.

## Runtime lifecycle

Spawned once, after the USB hosts and usb_hub0. It looks for a line every
second until one binds. While the line is held it looks at the device
every 5 ms between requests; otherwise it waits in the kernel.

## Failure model

A device whose bulk pipes, configuration or line coding is refused is not
bound and is tried again. A refused notification endpoint leaves the line
working without DCD, DSR or errors. A halted bulk IN pipe is cleared. A
holder the kernel says is gone is let go within a second. A device that
leaves its port is given back and its holder let go.

## Release target

USB serial consoles (boards that speak CDC ACM, debug adapters) and USB
modems' AT ports, from the terminal.

## Release evidence

`cdc_acm_proofs` covers the function finding, line coding and requests,
SERIAL_STATE and the state a client reads, the holder, the read buffer and
the NACM framing. The held-table proofs in `kernel_proofs`. No boot yet.

## Release checklist

- Signed by the owner and added to an image's features.
- A boot with a board that enumerates as ACM. QEMU's `usb-serial` is an
  FTDI chip, not ACM (as recalled, unverified), so it is no test of this.

## Real hardware bring-up checklist

- A microcontroller board as ACM: bytes both ways at 115200 and at 1 Mbit/s.
- A USB modem's AT port: `AT` answered `OK`, DCD seen when a call connects.
- Unplugging while held: the holder sees -19, replugging binds again.

## Explicit non-goals today

No vendor serial chips (FTDI, CP210x, PL2303, CH34x), no devices without a
Union descriptor, no more than one line, no encapsulated commands, no
MBIM or NCM data for modems.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/cdc_acm_proofs && cargo test --release)` and
`(cd userland/kernel_proofs && cargo test --release)`.
