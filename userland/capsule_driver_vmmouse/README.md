# capsule_driver_vmmouse

## Role

`capsule_driver_vmmouse` drives the VMware absolute pointer (vmmouse):
VMware's, and QEMU's on its default PC machines through the vmport device.
With it on, the guest's pointer follows the host's without a capture. It
reaches the pointer through the backdoor calls the kernel allows for it
alone, and posts the host pointer's position, buttons and wheel onto the
kernel input ring in the shared absolute range.

```text
host pointer --backdoor (MkHvBackdoor)--> driver.vmmouse0 --POINTER_ABS, BUTTON_*, WHEEL--> kernel ring --> input_router
```

## Microkernel contract

- `MkHvBackdoor` for GETVERSION (10), ABSPOINTER_DATA (39),
  ABSPOINTER_STATUS (40) and ABSPOINTER_COMMAND (41). The kernel allows
  those alone, and only on VMware, KVM or QEMU TCG; any other host answers
  `ENODEV`.
- `MkInputEventPost` for each position, button and wheel step. No
  `MkIrqBind`: the driver polls the pointer's status every 4 ms.
- No device is claimed: the backdoor is the hypervisor's port, not a PCI
  function. The kernel spawns the driver only when GETVERSION answers with
  the backdoor's magic.
- `service:4300:driver.vmmouse0` is held to the kernel alone; nothing calls
  the driver.

## Interface contract

No IPC is served. The driver posts, per packet and in this order:
- `INPUT_KIND_POINTER_ABS` with `x`, `y` from 0 to `INPUT_ABS_MAX` (libc
  `input_abs_scale` from the host's 0..0xFFFF), once per new position;
- `INPUT_KIND_POINTER_REL` for a relative packet, with Y down;
- `INPUT_KIND_WHEEL` with a notch away from the user as +1;
- `INPUT_KIND_BUTTON_DOWN`/`UP` for each changed button, 1 left, 2 right,
  3 middle.

## Authority

`IPC`, `Memory`, `Pio`, `InputSource` (`CAPSULE_REQUIRED_CAPS = 0x300018`);
`Debug` optional (`CAPSULE_OPTIONAL_CAPS = 0x100`). No `Driver`,
`DeviceEnum`, `Mmio`, `Dma`, `Irq`, `Network` or `FileSystem`.

```text
allowed:   the absolute pointer's backdoor calls; posting pointer input
forbidden: the backdoor's RPC channel (guest info, tools version, clipboard,
           display hints), host time, any other backdoor command
```

## Privacy and persistence

The host learns that the guest enabled the absolute pointer and asked for
absolute positions, and nothing else. The kernel refuses the RPC channel,
so no guest name, tools version, clipboard or other data reaches the host.
Every host reply is bounded before use: at most 16 packets a look, only a
queue of whole four-word packets, and positions scaled into the shared
range. Nothing is stored.

## Runtime lifecycle

GETVERSION, with the magic back in EBX and a version in EAX (else the
driver says so and leaves, `EXIT_ABSENT`). Then, on the shared bounded
bring-up schedule:
1. the pointer disabled, so its queue starts empty;
2. enabled;
3. its status read, which must be free of error with a word queued;
4. that word read, which must be the pointer's identity, 0x3442554A;
5. absolute positions requested, last, because the host then stops moving
   the PS/2 pointer.

Then poll: status, and while whole packets are queued, four words each
read and posted.

## Failure model

A host whose pointer status flags an error, or whose queue is no whole
number of packets, has lost the pointer. The driver:
- turns it off;
- lets go of any button it still holds;
- after a second, enables it again, as Linux's vmmouse does;
- keeps trying each second while that fails, reading no packet meanwhile.

A bring-up that never takes ends the driver with `EXIT_GAVE_UP` after the
shared schedule; the PS/2 mouse keeps working throughout.

## Current implemented surface

- Absolute position, three buttons and the wheel from the host.
- Relative packets (a host with the pointer grabbed) posted as motion.

## Wire format

The backdoor: `in eax, dx` on port 0x5658 with EAX 0x564D5868 (`VMXh`),
EBX the argument, ECX the command; EAX to EDX come back. A packet is four
words: flags and buttons (left 0x20, right 0x10, middle 0x08, relative
0x10000), X, Y, and the wheel as a signed byte, +1 a notch toward the user.

## State ownership

The host owns the pointer; the capsule owns only the last position and
buttons it posted. The input router owns where the cursor is drawn.

## Operating rules

- Ask for absolute positions last, and only once the identity is through.
- Read no more words than the status says are queued, and only whole
  packets.
- Never ask the kernel for a backdoor call outside the pointer's.

## Release target

The pointer follows the host's in VMware Workstation, Fusion and ESXi, and
in QEMU's default `pc` and `q35` machines, without a capture.

## Release evidence

`vmmouse_proofs` runs the shipping source against a model of QEMU's
vmmouse behind the kernel's own allowlist of backdoor calls (included by
path). It checks:
- the bring-up calls in order;
- every host refused, and a port that does not answer;
- a vmport with no pointer, a wrong identity, a pointer that queues
  nothing or stays in error, and one left on with packets queued;
- positions scaled and posted once, and again after motion;
- buttons, the wheel's sign, relative motion, and a position past the grid;
- the bound on one look;
- a lost pointer brought back, and one that stays lost;
- the console lines.

A boot is the owner's.

## Release checklist

- Signed manifest and kernel mirror present (`driver_vmmouse` keys).
- `vmmouse_proofs` passes.
- VMware or QEMU (`-machine pc`, vmport on): the pointer follows the host's
  without a capture, and clicks and the wheel land under it.

## Explicit non-goals today

No RPC channel (no tools version, guest info, display hints, clipboard or
shared folders), no host time, no interrupts (the PS/2 IRQ the host raises
for each packet is left to the PS/2 driver), no relative mode request.

## Verification

- Build: `make -B nonos-mk-driver-vmmouse`
- Proofs: `cd userland/vmmouse_proofs && cargo test --release`
- Static gate: `bash nonos-ci/run-static-checks.sh`
