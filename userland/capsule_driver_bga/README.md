# capsule_driver_bga

## Role

Bochs Graphics Adapter (QEMU stdvga 1234:1111, bochs-display) display
driver, serving the display wire (NVGP v1, PLAN-coverage section 12) on
`nonos_gfx_serve`. It keeps the mode firmware left until the compositor
asks for another, so it no longer destroys the firmware scanout that kept
`DisplayBga` undispatched (`src/hardware/inventory/driver.rs`). It still
has no `Capsule.mk`, Cargo feature or kernel mirror: the image carries it
once its trust artifacts are sealed (HANDOFF letter G-12).

## Microkernel contract

No production spawn contract is active. The spawn it is written for:
service `driver.bga0` on port 4240, IPC | Memory | GraphicsSurfaceCreate |
DeviceEnum | Driver | Mmio (`CAPSULE_REQUIRED_CAPS` 0x39018); no Dma (the adapter masters nothing),
no Irq (it polls nothing either: every wire request is answered at once),
no Pio (the DISPI block is read at BAR2+0x500). It uses brokered
`MkDeviceList`, `MkDeviceClaim`, `MkMmioMap`, surfaces and IPC only.

## Interface contract

The display wire as a single-output DISPI device (`kind` 2): CAPS
(MODE_SET | MODE_RANGE, 640x480 to 4096x4096, widths a multiple of 8, no
hardware cursor), MODES (the EDID's modes that fit, and the one shown),
SURFACE, MODE_SET, FLUSH, STATE, RETIRE; CURSOR answers ENOSYS.

## Authority

None while unspawned. Spawned, it holds the adapter's claim, BAR0 (video
memory, write-combining) and BAR2 (EDID and DISPI, uncached).

## Privacy and persistence

No user data is read or persisted.

## Runtime lifecycle

Not in the production spawn set. Spawned, it serves until stopped; it
sleeps in `mk_ipc_recv_from` between requests and holds no core.

## Failure model

Bring-up fails closed when a broker claim, an MMIO map or the first mode
fails. A MODE_SET the adapter does not take is read back, refused with
EINVAL and the previous mode put back.

With no BGA adapter in the device list the capsule logs one line and exits
`EXIT_ABSENT` (2) before claiming anything. An adapter that is present but
fails setup is retried on the shared bounded schedule
(`nonos_libc::bring_up`: seven tries, sleeping between them, each failed try
having released its claim); running out exits `EXIT_GAVE_UP` (6).

## Current implemented surface

Discovery, claim and maps; the EDID read at BAR2+0 (`edid/`); the DISPI
mode set and read-back (`dispi/`); the start-up mode (firmware's, else the
EDID's preferred, else 1024x768: `setup/mode.rs`); the wire's device half
(`display/`) and kernel half (`nonos_gfx_pool`), served by `serve.rs`.

## Wire format

```text
compositor -> driver.bga0 (port 4240): NVGP v1 frame -> reply frame
```

## State ownership

Each primary is anonymous memory of this capsule's, registered as an
ARGB8888 surface; FLUSH copies its rectangles into video memory and fences
the copy before the answer. Video memory itself is never shared.

## Operating rules

Do not add raw MMIO or framebuffer access outside broker grants.

## Release target

Brokered BGA display driver on the display wire, in the image's spawn set.

## Release evidence

Static broker-boundary audit plus hardware or emulator display proof.

## Release checklist

- Add Capsule.mk with the contract above.
- Add the kernel mirror under `src/hardware/` and its spawn.
- Seal and regenerate trust artifacts (owner only).
- Dispatch `DisplayBga` in the hardware inventory.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2); the capsule's own `no controller present, not started` line needs the Debug capability, so without it the kernel prints `[EXIT] <service> status 2: no device present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- (Bochs or QEMU stdvga only.) Firmware's picture stays until the compositor's first FLUSH; with DISPI left off, the EDID's preferred mode (QEMU's default 1280x800; unverified) is set.
- A MODE_SET to each EDID mode changes the picture in one step, with no black frame.
- Between requests the capsule is asleep: no CPU time accrues to it.

## Explicit non-goals today

No hardware cursor, no display hints (Decision 15), no second output, and no
use of the adapter outside QEMU and Bochs.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/bga_proofs && cargo test --release)`.
Handbook: [drivers](../../docs/handbook/drivers.md).
