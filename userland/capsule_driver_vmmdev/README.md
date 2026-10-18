# capsule_driver_vmmdev

## Role

`capsule_driver_vmmdev` drives VirtualBox's guest device (PCI 80ee:cafe)
for one thing: mouse integration. With it on, the guest's pointer follows
the host's without a capture, the way the virtio tablet does in QEMU. It
introduces the guest, asks the host for the mouse's events alone, offers
the host no capability, and posts each new host pointer position onto the
kernel input ring in the shared absolute range.

```text
host pointer --VMMDev event--> driver.vmmdev0 --INPUT_KIND_POINTER_ABS--> kernel ring --> input_router
host buttons, wheel --PS/2 mouse--> driver.ps2_kbd0 --> kernel ring --> input_router
```

## Microkernel contract

- `MkDeviceList` finds the device: 80ee:cafe with an I/O BAR0 (the request
  port) and a memory BAR1 (VMMDevMemory, the event page).
- `MkDeviceClaim`; `MkPciConfigWrite` sets memory decode, Bus Master and
  Interrupt Disable.
- `MkPioGrant` on BAR0; `MkMmioMap` of BAR1's first page; `MkDmaMap` of one
  coherent page below 4 GiB for the requests.
- `MkInputEventPost` for each position. No `MkIrqBind`: the driver polls
  the event page every 4 ms.
- `service:4284:driver.vmmdev0` is held to the kernel alone; nothing calls
  the driver.

## Interface contract

No IPC is served. The driver posts `INPUT_KIND_POINTER_ABS` with `x`, `y`
from 0 to `INPUT_ABS_MAX` (libc `input_abs_scale` from VMMDev's 0..0xFFFF),
once per new position, and only while the host's integration is on.

## Authority

`IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Dma`, `Pio`, `InputSource`
(`CAPSULE_REQUIRED_CAPS = 0x3B8018`); `Debug` optional
(`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Irq`, `Network` or `FileSystem`.

```text
allowed:   VMMDev's request port, event page, one request page; posting pointer positions
forbidden: HGCM (shared clipboard, folders, guest properties), display hints,
           seamless mode, ballooning, credentials, host time
```

## Privacy and persistence

The host is told the interface version, an unknown 64-bit guest, no
capabilities and that the guest takes absolute positions. No Guest
Additions version or name is claimed, no facility status reported, and no
HGCM connection made, so the host can neither push nor pull clipboard,
files or properties. Nothing is stored.

## Runtime lifecycle

Claim and grants; the event page checked to be VMMDevMemory (version 1,
size at least 32 and within BAR1); the host's version asked (a host that
does not answer is no VMMDev); the guest info; the event filter set to the
mouse's position event alone; every capability cleared; events already
pending cleared; mouse integration turned on last. Then poll: when the page
flags events they are acknowledged, and a position event is followed by a
mouse status read and a post.

## Failure model

Every bring-up step names its failure and gives back the request page, the
event page, the port grant and the claim, newest first. Integration is
turned on last because the host then stops moving the PS/2 pointer: a
bring-up that failed after it would leave the guest without one. A
request the host refuses at runtime is skipped; the next event carries the
position anew. Bring-up is tried a bounded number of times
(`nonos_libc::start_driver`).

## Current implemented surface

- Absolute pointer position from the host, with buttons and wheel left to
  the PS/2 mouse, which VirtualBox keeps feeding them.

## Wire format

VMMDev requests: a 24-byte header (size, version 0x10001, type, status,
reserved, requestor) and the body, at the start of the request page; the
page's 32-bit bus address written to the request port carries it out.

## State ownership

The capsule owns the device and the request page. The input router owns
where the cursor is drawn.

## Operating rules

- Ask for no event and offer no capability beyond the mouse's.
- Turn integration on last, and only once everything else is through.
- Never connect to an HGCM service.

## Release target

The pointer follows the host's in VirtualBox without a capture.

## Release evidence

`vmmdev_proofs` runs the shipping source against a model of VMMDev that
checks every request's header and exact size and refuses all but the host
version before the guest info: the grant order, the introduction and its
order, every refusal and what it gives back, integration never turned on
after a failure, positions scaled and posted once, filtered events never
flagged, and integration off. A boot is the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_vmmdev` keys).
- `vmmdev_proofs` passes.
- VirtualBox with mouse integration: the pointer follows the host's without
  a capture, and clicks land under it.

## Explicit non-goals today

No display change hints (the compositor takes its size once), no seamless
mode, no HGCM services, no ballooning, no host time, no heartbeat.

## Verification

- Build: `make -B nonos-mk-driver-vmmdev`
- Proofs: `cd userland/vmmdev_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
