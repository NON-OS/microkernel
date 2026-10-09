# capsule_driver_bga

## Role

Parked Bochs Graphics Adapter capsule source. It has no `Capsule.mk`, no
Cargo feature and no kernel mirror, so no image carries it. The kernel's
hardware inventory leaves `DisplayBga` undispatched on purpose: the capsule
re-modes the adapter to 1024x768x32 and would destroy the firmware scanout the
compositor falls back to (`src/hardware/inventory/driver.rs`).

## Microkernel contract

No production spawn contract is active. A promoted version must declare
`CAPSULE_REQUIRED_CAPS` and use brokered `MkDeviceList`, `MkMmioMap` and IPC
only.

## Interface contract

No stable service endpoint is exported while parked.

## Authority

Parked status means no granted runtime authority.

## Privacy and persistence

No user data is read or persisted.

## Runtime lifecycle

Not in the production spawn set.

## Failure model

Promotion must fail closed when broker claims, MMIO mapping or display setup
fails.

With no BGA adapter in the device list the capsule logs one line and exits
`EXIT_ABSENT` (2) before claiming anything. An adapter that is present but
fails setup is retried on the shared bounded schedule
(`nonos_libc::bring_up`: seven tries, sleeping between them, each failed try
having released its claim); running out exits `EXIT_GAVE_UP` (6). Once the
mode is set the capsule sleeps; it holds no core.

## Current implemented surface

Source inventory for a future brokered BGA display capsule.

## Wire format

```text
client -> parked bga service -> no production endpoint
```

## State ownership

No production surface ownership while parked.

## Operating rules

Do not add raw MMIO or framebuffer access outside broker grants.

## Release target

Brokered BGA display driver capsule with explicit manifest capabilities.

## Release evidence

Static broker-boundary audit plus hardware or emulator display proof.

## Release checklist

- Add Capsule.mk.
- Declare `CAPSULE_REQUIRED_CAPS`.
- Register signed spawn path.
- Regenerate trust artifacts.

## Real hardware bring-up checklist

What only a boot on real silicon can confirm. The host proofs cover the parsers, the bring-up sequence against a model, and the bounded retry; these do not.

- On a machine without the device: the broker log shows no claim for it and the capsule exits at once with `EXIT_ABSENT` (2); the capsule's own `no controller present, not started` line needs the Debug capability, so without it the kernel prints `[EXIT] <service> status 2: no device present, not started` for it (`src/process/exit/end_note.rs`).
- With the device present but failing (disabled in firmware, or a forced setup error): at most seven claim and release rounds in the broker log over about six seconds, then one `device present, bring-up failed` line naming the last cause (with the Debug capability), and exit `EXIT_GAVE_UP` (6), which the kernel names as `[EXIT] <service> status 6: device present, bring-up failed and was given up`. The capsule holds no core while it waits.
- (Bochs or QEMU stdvga only.) The 1024x768x32 mode is set and the framebuffer cleared to the boot colour.
- Once the mode is set the capsule is asleep: no CPU time accrues to it.

## Explicit non-goals today

No production display backend is claimed from this capsule today.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
Proofs: `(cd userland/bga_proofs && cargo test --release)`.
Handbook: [drivers](../../docs/handbook/drivers.md).
