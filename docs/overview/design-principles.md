# Design principles

The rules the NONOS code follows, each with the code or the check that holds it, and the places where it does not hold yet.

## Least privilege by capability word

Every process holds a [capability word](glossary.md#capability-word), a mask over the 36 capability bits the kernel defines (`capability_table` in `src/capabilities/types/defs.rs:21-83`). A [capsule](glossary.md#capsule)'s word comes from its signed [manifest](glossary.md#manifest): its required bits, plus those of its optional bits the spawning code allows (`install_caps` in `src/security/capsule_manifest/verify/caps_bits.rs:38-47`). At spawn, a grant outside the manifest is refused with `GrantOutsideManifest` (`check_grant` in `src/security/capsule_manifest/verify/caps.rs:31-39`). Every syscall the kernel knows then passes `dispatch`, which refuses with EPERM when the caller's capability does not resolve for that call (`src/syscall/contract/dispatch.rs:25-40`).

Services hold the same line. The file store answers only the kernel and holders of FileSystem (`CAP_FILE_SYSTEM` in `userland/capsule_vfs/src/server/fs_gate.rs:17-40`). The NVMe, AHCI and virtio-blk drivers serve raw sectors only to the kernel's own client and to holders of StoreWrite (`permits` in `userland/capsule_driver_nvme/src/server/medium.rs:17-30`). Each asks the kernel on every request instead of caching the answer, so a verdict never outlives the process it was about.

The checks: `scripts/cap_audit.py --strict` refuses a capsule whose own code makes a call its mask has no bit for, except the two capsules in `EXEMPT`, `attack` and `toolkit` (`scripts/cap_audit.py:66-72`). It reports a granted bit with no call behind it but does not refuse one. `scripts/check_mirror_caps.py` holds each kernel spawn file and README to its manifest. `nonos-ci/run-static-checks.sh` runs both inside the `static-tree` flake check. On this commit the strict audit prints `107 capsules, 8 granted bits with no evidence, 1 capsules missing a bit a call needs, 0 strict failures (attack, toolkit exempt)`, and the capsule missing a bit is `toolkit`. The mirror check reports no mask wider than its manifest. `static-tree` still fails on this commit, on another rule. It refuses an import named `read` or `write` in a listed set of capsules, since such names look like the Linux calls (`forbidden_import_dirs` in `nonos-ci/run-static-checks.sh:4493-4512`), and the AHCI driver's eMMC handler imports its own `read` and `write` (`userland/capsule_driver_ahci/src/server/handlers/emmc/dispatch.rs:29`).

The limits: two bits, `IO` and `Hardware`, enforce nothing (`src/capabilities/types/defs.rs:23-31`), and two more, `Keyring` and `Entropy`, gate only kernel clients that nothing calls ([ABI capabilities](../abi/capabilities.md#checks-outside-the-syscall-table)). Exiting, yielding, futex waits, reading the clock and reading process stats take no particular bit, only a valid [capability token](glossary.md#capability-token), which is one that holds at least one bit (`check` in `src/syscall/contract/cap_table/mk.rs:20-35`). After spawn, a process that holds Admin can add to another process any bit it holds itself, whether or not that process's manifest declares it (`sys_cap_grant` in `src/syscall/microkernel/capability/handlers.rs:33-52`).

## Drivers run in ring 3

A device driver is a capsule like any other, with no more reach than its bits allow. It touches its device only through the [hardware broker](glossary.md#hardware-broker): claiming a device needs Driver, mapping its registers needs Mmio, and a DMA buffer needs Dma (`MkDeviceClaim`, `MkMmioMap` and `MkDmaMap` in `src/syscall/contract/cap_table/mk.rs:124-131`). On Intel VT-d machines the [IOMMU](glossary.md#iommu) confines device DMA: each PCI device a driver claims moves into that driver capsule's own domain, which maps only the buffers granted to it, and a claim the unit in service cannot confine is refused (`attach` in `src/hardware/broker/confine/attach.rs:30-101`). A driver that crashes ends as a process, and the process teardown releases its claims and grants on the way out (`release_all_for_pid` in `src/process/exit/teardown.rs:47-51`).

Of device code, the kernel keeps PCI enumeration and a virtio-rng entropy probe for its own boot (`init_pci` and `init_virtio_rng` in `src/drivers/mod.rs:17-35`), plus the interrupt controllers, the timers and the TPM.

The limits: every device a capsule claims shares its one domain, so each can reach the buffers granted for the others (`map` in `src/hardware/broker/confine/map.rs:25-50`). AMD-Vi is not driven and interrupt remapping is off: no [build profile](glossary.md#build-profile) turns on `nonos-iommu-amdvi` or `nonos-iommu-intremap` (`Cargo.toml:879-891`). With AMD-Vi, or with no remapping unit in service at all, a claimed device can reach all of physical memory (`unconfined_allowed` in `src/hardware/broker/confine/posture.rs:17-50`). A device found at boot that no driver claims stays in the [identity domain](glossary.md#identity-domain), and a device with no PCI requester id gets no domain (`pci_address` in `src/hardware/broker/confine/table.rs:35-43`).

## Everything that runs is signed and measured

A capsule runs only after `spawn_verified` has passed it through `preflight`. Before that, the profile gate refuses a capsule the boot's menu entry rules out, such as a network driver on an Air-Gapped boot (`check` in `src/kernel_core/process_spawn/capsule_spawn/runner/profile_gate.rs:17-31`).

```mermaid
sequenceDiagram
    participant S as spawn_verified
    participant P as preflight
    participant G as attest_gate
    participant R as record_attested
    S->>P: certificate, manifest and ELF
    P->>P: verify_id_cert, then verify_with_publisher
    P->>G: the capsule and its trailer
    G-->>P: measurement and authority, or AttestationRejected
    P-->>S: install_caps from the manifest
    S->>R: pid, measurement and capabilities
```

1. `verify_id_cert` checks the capsule's [NONOS ID certificate](glossary.md#nonos-id-certificate) against the [trust anchor](glossary.md#trust-anchor) under `NONOS_PRODUCTION_POLICY`, which requires both Ed25519 and ML-DSA-65 (`src/security/nonos_id_cert/policy.rs:30-32`).
2. `verify_with_publisher` checks the manifest and the ELF image under the same policy and returns `install_caps`, the capabilities to install (`src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:55-67`).
3. The capsule and its trailer then pass `attest_gate`: an empty [attestation trailer](glossary.md#attestation-trailer), or one whose proof does not verify, ends the spawn with `AttestationRejected` (`src/kernel_core/process_spawn/capsule_spawn/runner/attest_gate.rs:23-34`). A capsule in the `systems.nonos` namespace goes straight to it; any other is in the publisher tier and reaches the same gate through `publisher_gate` (`src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:72-77`, `src/kernel_core/process_spawn/capsule_spawn/runner/publisher_gate.rs:20-33`).
4. Once a pid exists, `record_attested` writes the capsule's measurement and capabilities into the attestation registry (`src/security/attest_registry/record.rs:31`). The measurement is the BLAKE3 digest of the ELF (`measure` in `src/security/capsule_attest/measure.rs:17-26`).

The [Linux personality](glossary.md#linux-personality) holds Linux programs to the same rule. A program read from the store runs only if the trailer kept beside it verifies, and an exec of one that does not gets EPERM (`resolve` in `userland/capsule_linux/src/linux/call/spawn/exec_resolve.rs:40-60`). The built-in BusyBox is part of the personality's own measured image.

The kernel image comes before any of this. The loader's menu names Ed25519, ML-DSA-65, STARK, rollback and RNG as the checks for every entry that boots (`STD` in `nonos-bootloader/src/bootmenu/entries.rs:58-59`), and [Boot chain and signatures](../security/boot-chain-and-signatures.md) covers the loader's side. The loader itself carries no NONOS signature: it is signed only for [Secure Boot](glossary.md#secure-boot), when the [seal](glossary.md#seal) holds a db key. It is held to its own [STARK proof](glossary.md#stark-proof) slot instead, which the kernel checks under the signed [boot-root record](glossary.md#boot-root-record) before any program starts (`check_bootloader` in `src/kernel_core/init/entry/microkernel_main.rs:24-27`).

The limit: a `dev` image admits capsules on their path alone, without the STARK proof, and the seal refuses it for release (`tools/nix/config.nix:102-109`). Its loader is built with `dev-attest`, which the comment above it describes as taking the kernel's path-only trailer the same way, and with `dev-mode`, which boots an unsigned kernel when F12 is held and Secure Boot is off (`dev-qemu` in `nonos-bootloader/Cargo.toml:68-72`).

## Amnesic by default

Every boot is [amnesic](glossary.md#amnesic-boot) unless the person chooses to install, in first-boot setup or from the boot menu's install entry. With the `install` key set to false, an image has no setup and no installer at all (`nonos.toml:22-25`). First-boot setup is the only thing that records the choice to keep anything (`installFeatures` in `tools/nix/config.nix:43-46`). When setup ends, init hands the screen to the installer only if the person chose to install, or if setup ended without an answer on a boot started from the menu's install entry (`poll` in `src/userspace/init/supervisor/after_setup.rs:17-48`).

A file a program asks to keep goes to the [package store](glossary.md#store), which is written to the disk as it is, with no encryption (`append` in `userland/capsule_vfs/src/blk/store_write.rs:39-84`). Qwen models and other imported files go to a [data volume](glossary.md#data-volume). On a live stick, whose disk plan keeps no volume, that volume is held in RAM (`open_session_volume` in `src/fs/blockfs_volume/open_machine.rs:46-53`). On an installed disk every sector is sealed with ChaCha20-Poly1305 (`seal` and `aead_encrypt` in `src/fs/cryptoblock/seal.rs:18-46`), under a [machine key](glossary.md#machine-key) the [TPM](glossary.md#tpm) derives for this machine in this boot state (`KEY_LABEL` in `src/fs/blockfs_volume/open_machine.rs:17-38`). The kernel can instead seal a random volume key with a passphrase stretched by Argon2id (`passphrase_volume` in `src/fs/blockfs_volume/passphrase.rs:17-32`), but no capsule in this release asks for one.

Shutting down and restarting, the `AdminShutdown` and `AdminReboot` syscalls, both run the [ZeroState](glossary.md#zerostate) wipe first (`shutdown` in `src/syscall/dispatch/router/admin/shutdown.rs:20-27`, `reboot` in `src/syscall/dispatch/router/admin/reboot.rs:20-28`): the other CPUs are stopped, then memory is wiped, then the firmware is called (`terminate` in `src/security/zerostate/terminate.rs:21-39`). The wipe stops claimed devices and wipes their DMA buffers before it wipes process memory, kernel stacks, filesystem caches, the kernel's key vault, the RAM log and the heap (`zerostate_shutdown_wipe` in `src/security/hardening/memory_sanitization/api.rs:59-109`).

The limits: stopping the other CPUs is best effort. On an interrupt controller that refuses the broadcast, the wipe covers what the running core can reach (`terminate` in `src/security/zerostate/terminate.rs:31-35`). Kernel statics outside the heap are not wiped, the data volume key in `VOLUME` among them (`src/fs/blockfs_volume/state.rs:21-26`). A live stick's in-memory volume is not wiped either: `Ram` keeps its sealed sectors in frames outside the heap (`src/fs/cryptoblock/ram.rs:41-47`). In 0.9.2 the desktop cannot shut down or restart, so the installer's restart is the one path that runs the wipe (`mk_admin_reboot` in `userland/capsule_install/src/install/event/after.rs:38`). A kernel panic halts every CPU without the wipe (`panic` in `src/boot/panic/handler.rs:41-64`), and cutting the power skips it too.

## Refuse rather than fall back

When something cannot be done safely, the code refuses and says why, instead of taking a weaker path.

- No route beats the wrong route. A chosen network that is not running gives `Down`, never `Direct` (`pick` in `userland/nonos_route_link/src/pick.rs:125-139`). The wallet's chain reads go over Nym or Anyone even when Direct is chosen (`private_only` in `userland/nonos_route_link/src/pick.rs:86-100`).
- No factory address on the wire. The e1000 driver fails when it has no randomness for a station address, rather than fall back to the address in its EEPROM (`draw` in `userland/capsule_driver_e1000/src/init/station_address.rs:17-32`).
- No clock sync that names the machine. net.ntp asks a time server only when Direct is chosen (`step` in `userland/capsule_net_ntp/src/decide.rs:17-45`). No image built from a build profile starts it: init has `spawn_ntp` only when `nonos-capsule-net-core` is off (`src/userspace/init/spawn_plan/network/mod.rs:28-29`), and no build profile turns on `nonos-capsule-net-ntp` (`tools/nix/config.nix`). The one kernel that carries it, the `nonos-mk-ntp-test` smoke test, has no [policy store](glossary.md#policy-store), so net.ntp there cannot read Direct and asks nothing.
- No guessed CPU. The kernel runs on every core, and a CPU whose APIC id is in no descriptor halts rather than run as the boot CPU with another CPU's process and authority (`cpu_id` in `src/smp/cpu_id.rs:32-43`).
- No guessed Linux call. A call the Linux personality does not serve answers ENOSYS and names itself on the log (`unserved` in `userland/capsule_linux/src/linux/serve/unserved.rs:21-41`).

## Small files, and mod.rs only declares

The house rule is a file of at most 75 lines that does one thing, and a `mod.rs` that only declares modules and re-exports them; `verification/ASSUMPTIONS.md` calls it the 75-line rule. `nonos-ci/run-static-checks.sh` enforces it for the bootloader: a bootloader `mod.rs` with anything but declarations fails (`boot_mod_bodies` at `nonos-ci/run-static-checks.sh:2217-2237`), and so does one past 75 lines (`boot_mod_oversize` at `nonos-ci/run-static-checks.sh:2240-2246`).

In the kernel it is a convention, not a check. 790 of the 5,761 Rust files under `src/` are longer than 75 lines, and 36 of its 841 `mod.rs` files define a function. Counted from the repository root:

```sh
find src -name '*.rs' -print0 | xargs -0 wc -l | awk '$2 != "total" && $1 > 75' | wc -l      # 790
find src -name mod.rs | wc -l                                                                # 841
grep -lE '^\s*(pub(\([a-z]+\))? )?(const )?(unsafe )?fn ' $(find src -name mod.rs) | wc -l    # 36
```

## Proofs live next to the code

A [proof crate](glossary.md#proof-crate) mounts the shipped source by `#[path]` and tests it on the host with `cargo test`. For example, `kernel_proofs` mounts the kernel's service registry policy (`service_policy` in `userland/kernel_proofs/src/lib.rs:136-138`). `nix flake check` runs every `*_proofs` crate under `userland/` that has a lock file, the bootloader's `boot_proofs` and six more host crates (`proofDirs` in `tools/nix/checks.nix:18-29`). Each runs its tests with overflow checks on in the release build, then clippy with warnings as errors (`proof` in `tools/nix/checks.nix:70-97`). Clippy skips three crates that are not clean yet and lints only the library of eight more (`lintNone` and `lintLib` in `tools/nix/checks.nix:44-62`).

The flake defines 114 proof-crate checks. In the check run on this commit, 113 were built and 110 passed; `proofs-model_fetch_proofs` did not evaluate, as [CI](../build/ci.md) explains. `rtl8169_proofs`, `usb_msc_proofs` and `xhci_proofs` failed at their clippy step; `rtl8169_proofs` had passed its 67 tests first.

The Lean 4 models in `verification/lean/` are not built by `nix flake check`. Its `static-evidence` check requires the `sorry_count` that `verification/evidence/collect-evidence.sh` finds in their sources to be zero (`tools/nix/checks.nix:240-246`), and it passes on this commit. Kani, Verus, the Charon and Aeneas extraction and the fuzzers run outside the flake, as the header of `tools/nix/checks.nix` says. Everything NONOS trusts without proof is listed in [verification/ASSUMPTIONS.md](../../verification/ASSUMPTIONS.md).

The limit: proof crates test code on the host; they do not boot it. What has been seen on a machine is in the [support matrix](../hardware/MATRIX.md).

## See also

- [Overview](README.md)
- [Architecture](architecture.md)
- [Threat model](threat-model.md)
- [Code style](../contributing/code-style.md)
- [Tests and proofs](../contributing/tests-and-proofs.md)
- [Capsule isolation](../security/capsule-isolation.md)
