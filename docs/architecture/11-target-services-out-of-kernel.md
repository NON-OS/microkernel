# Target kernel: services out of the kernel

How the kernel-side IPC clients, the in-kernel name service and the init supervisor leave ring 0.

## The kernel as a client of its own capsules

The kernel talks to userland capsules through its own RPC transport in `src/services/lifecycle/` (`transport.rs` and the `trip_*`, `reply_*` and `waiting` files): a 20-byte frame header, request identifiers, a reply inbox the kernel drains, and a lock held across the round trip. While it waits, the system call that started the round trip blocks. This puts three things in ring 0 that a microkernel avoids: waiting on userland, parsing replies a capsule wrote, and holding a kernel lock across both.

Who uses it, at `551bbd34`:

| Client | Kernel caller | Status | Destination |
|---|---|---|---|
| nvme, ahci, virtio_blk, usb_msc (`src/hardware/*_capsule/client`) | The block layer, for example `src/hardware/block_device/rows/nvme.rs:32-35` | Live | **storage**, with the block layer |
| crypto (`src/security/crypto_capsule/client`) | The crypto system calls, which forward hash, AEAD, HMAC, HKDF and X25519 (`src/security/crypto_capsule/client/transport.rs:23`) | Live | Deleted: capsules call the crypto capsule directly |
| entropy (`src/security/entropy_capsule/client`, `fast.rs`) | `CryptoRandom` (`src/syscall/dispatch/crypto/random.rs:36`) | Live | Deleted: the kernel's own RNG serves `KRND`; capsules that need more ask the entropy capsule |
| keyring (`src/security/keyring_capsule/client`) | The data volume (`src/fs/blockfs_volume/mount_volume.rs:24`) | Live | **volume** |
| market (`src/security/market_capsule/client`) | Init's Linux jobs (`src/userspace/init/linux_jobs/vouched.rs:24`, `src/userspace/init/linux_jobs/box_of.rs:30`) | Live | **init**, which is then a userland client of the market capsule like any other |
| e1000, xhci, hda, ps2_kbd, rtl8139, rtl8169, virtio_net, virtio_rng | none | Dead | Deleted |
| vfs, ramfs (`src/fs/vfs_capsule/client`, `src/fs/ramfs_capsule/client`) | Only the never-mounted in-kernel fd layer | Dead | Deleted |
| usb_hid (`src/userspace/capsule_driver_usb_hid/client`) | none | Dead | Deleted |

Each live client moves with its caller, so no client remains in ring 0 and `services/lifecycle/transport*` is deleted with them. What changes for the callers is that they are now userland services holding endpoint capabilities to the capsules they call, received from init.

## The service registry

`src/services/registry.rs` is a name service in ring 0: up to 256 named endpoints with a port, an owner pid and the capability bits a sender needs (`src/services/registry.rs:58-80`), plus reserved names, peer lists, holder tables for displays, NICs and USB hosts, and the Network rule (`src/services/registry/policy.rs:26-38`). It is consulted by IPC sends, by the spawn gate and by the syscall gate.

In the target the kernel has no names. The **names** service holds one endpoint capability per registered service and answers a lookup with a copy of it, minted with a badge that identifies the asker. What today's `caps_required` expresses (only a holder of bit X may send to service Y) becomes a capability-class rule: an endpoint created by a service with the Network class can be copied only into processes whose class mask includes Network ([07-target-object-model.md](07-target-object-model.md)). Peer lists become the set of capabilities init hands out. Services that guard the system's trust (verifier, tpm, devmgr) are not registered by name; init passes their capabilities to the few processes that need them.

## Init

### Today

Init is ring-0 code. `microkernel_main` creates a process named `init`, gives it an address space and a kernel stack, and calls `run_init()` as a function (`src/kernel_core/init/entry/microkernel_main.rs:42-61`). `run_init` spawns the boot set in a fixed order, from ramfs (`src/userspace/init/entry.rs:28`) to the init loop (`src/userspace/init/entry.rs:51`); each spawn goes through `spawn_plan::boot::capsule` (`src/userspace/init/spawn_plan/boot.rs:24`) and `capsule_boot::run` (`src/userspace/init/capsule_boot/run.rs:27`). The loop ticks the kernel's lifecycle supervisor, the watch list and the late driver stages once a second, and serves instance spawns, ACPI power-off, installs and prover jobs (`src/userspace/init/supervisor/loop_impl.rs:35-67`). The restart rule watches 32 services (`src/userspace/init/supervisor/watch_rule.rs:28`), decides with `next` (`src/userspace/init/supervisor/watch_rule.rs:87`) and caps restarts at eight with a debounce (`src/services/lifecycle/state/respawn.rs:32`).

### Target

Init is the root task: the first userland process, started by the kernel from a flat image inside the signed kernel image ([09-target-spawn-and-boot-trust.md](09-target-spawn-and-boot-trust.md)).

| Today, in ring 0 | Target, in init |
|---|---|
| `spawn_plan` (17 stages, compile-time order, 131 spawn sites) | The same order as data in the boot package; each entry names an image, its artefacts, its endpoints and the capabilities init hands it |
| `capsule_boot::run`, `once::Tried` | The spawn protocol: seal, verify, load, `process_create`, `process_start` |
| `services::lifecycle::register` and `tick`, `CapsuleState` liveness cells | The exit endpoint of each child: init waits on a notification and learns of every death without polling |
| `watch_rule` and `respawn` | Unchanged as policy, now userland code; the host proofs of `watch_rule` move with it |
| `linux_jobs` (install, uninstall, run, run with network) | The Linux job handler in init, a userland client of the market capsule and of the personality |
| `instance_spawn`, `app_launch`, `tool_run`, `prove_job` system calls that enqueue work for init | IPC to init's endpoint, from the shell and the installer |
| `arch::acpi_from_init` power-off | `KPWR` on the power capability init holds |
| Reading the boot profile from the handoff | The boot profile as a field of the boot-info page |

### What init needs from the kernel

Everything init needs is in the target surface: address-space, thread and process creation, the verdict path, MemoryObjects for the boot package, endpoints and notifications, exit notification, `KPWR`, and a boot-info page describing the capabilities it starts with. The interim path, before the object model exists, is in [13-migration-plan.md](13-migration-plan.md) phase 2: init runs in ring 3 with today's `MkCapsuleLoad` path, which already spawns from bytes a caller hands in (`src/syscall/microkernel/capsule_load/handle.rs:29-61`), and the kernel maps the boot package read-only into init.

### Who supervises init

Nothing above it. If init dies, the kernel wipes RAM and resets, as it does today on a fatal kernel error. The boot set's critical services (verifier, devmgr, names) are pinned images, so a restart of init can re-create them identically.

## Per-capsule kernel mirrors

The 69 `src/userspace/capsule_*` directories, the 64 `src/hardware/*_capsule` directories, `src/fs/vfs_capsule`, `src/fs/ramfs_capsule` and the four `src/security/*_capsule` directories hold, for each capsule, an `embed.rs` with four constants, a `spawn.rs` with a spawn spec and a `state.rs` liveness cell. In the target each becomes one entry of the boot package and one entry of init's spawn plan, and the directories are deleted. The capsule's requested capability set, today written by hand in `spawn.rs` (for example the nvme driver's at `src/hardware/nvme_capsule/spawn.rs:51-58`), is taken from its signed manifest, which already bounds it.
