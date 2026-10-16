# bga_proofs

Host proofs for the Bochs Graphics Adapter capsule (`capsule_driver_bga`).
The crate includes the shipping driver source with `#[path]` and runs it
against a register window, a framebuffer and a surface in memory
(`nonos_devmodel`), with `libc_shim` standing in for the broker and the
kernel, without a boot and without an adapter. The shim's receive policy is
the libc's own `bringup` files. The shim records the DISPI block at every
give-back and poisons the register mapping from its unmap to the release
(`seal`), so a register written after its mapping is given back is seen.
The proofs' `regs` is the shipping file with each register write handed to
the test's adapter model as it is made (`hook`), so a model that answers a
write, as an adapter clamping a mode does, needs no thread and no race.

## What it proves

57 `#[test]` functions. The DISPI mode set and its register offsets
(`mode_tests`, `offset_tests`; a memory window cannot witness the write
order, only the settled state), the framebuffer clear (`clear_tests`), a
mode against the mapping it must fit at every bound (`span_tests`), the
DISPI ID accepted only from 0xB0C0 to 0xB0C5 and a mode or line layout that
does not read back as set refused (`check_tests`), and the failure reasons
(`reason_tests`). One bring-up attempt end to end (`setup_tests`,
`setup_bound_tests`, `setup_claim_tests`, `setup_confirm_tests`): Memory
Space on and Bus Master off, a function with no DISPI ID refused before a
mode is written, a short framebuffer or register mapping refused with
nothing written past it, a mode an adapter model clamps refused by name
with nothing drawn, and the handles giving back both mappings and then the
claim.

The driver up and serving (`display_tests`): get_primary's geometry and
format as the compositor checks them and the surface the driver's own
memory (`primary_tests`); the primary only the compositor's, the pid its
name is registered to, a respawned one's too (`owner_tests`,
`respawn_tests`); flush copying exactly the damaged rectangle and transfer
held to the offset flush reads (`flush_tests`); rectangles at and past
every edge, empty, and at u32::MAX (`edge_tests`, `rects`); set_scanout
taking only the whole mode and E_DEVICE once the adapter drops it
(`scanout_tests`); hostile frames and ops (`wire_tests`, `op_tests`); the
firmware's mode kept when drawable (`firmware_tests`) and its whole DISPI
state written back before the first unmap, at drop and on a failed attempt,
a ModeNotTaken one included (`restore_tests`, `not_taken_tests`), never
over a state unchanged and in QEMU's order (`write_back_tests`); nothing
drawn before the surface exists (`picture_tests`); the surface at the
kernel's 64 MiB bound and a page past it, and its memory refused
(`surface_tests`); VIDEO_MEMORY_64K too small refused by name
(`video_tests`); each receive waiting with a deadline, never sleeping in
its place (`turn_tests`); and the service name and port beside Capsule.mk
and the compositor's lookup order, virtio-gpu first, with its one log line
for each driver it finds (`service_tests`, which includes
`userland/compositor/src/setup/discover/order.rs`).

## Run

```sh
cd userland/bga_proofs && cargo test --release
```

See [drivers](../../docs/handbook/drivers.md) and
[proofs](../../docs/handbook/verification/proofs.md).
