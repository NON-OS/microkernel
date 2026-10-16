# Threat model: In scope

Part of [Threat model](threat-model.md).

## In scope

- A capsule, an app or a Linux program that tries to reach past its capability
  word.
- A device that tries DMA outside its capsule's grants, where a VT-d unit in
  service covers it.
- An observer on the network path, for connections made under Nym or Anyone.
- Someone who reads the disk of a machine that is off.
- A capsule changed after it was signed and enrolled.
- A kernel changed after it was signed and enrolled, a loader changed after it
  was enrolled that still hands the kernel the TPM's event log, and an older
  signed kernel put back on a machine whose TPM holds the rollback floor.

## Out of scope

- Firmware, a CPU or a TPM that lies. These are the assumptions above.
- An older signed kernel on a machine with no TPM, where every entry but
  Hardened and Air-Gapped boots with rollback protection off
  (`Floor::Unprotected` in
  `nonos-bootloader/src/boot/crypto/rollback/floor.rs:52-58`), or on one whose
  TPM was cleared, which starts the floor again at 0.
- A changed loader on a machine with no TPM, or one that hands the kernel no TCG
  log: the kernel's check of it then rests on the loader's own word (`decide` in
  `src/security/boot/loader_check/run.rs:34-55`).
- A whole image sealed with other keys, booted with Secure Boot off. Booting
  with Secure Boot on is not tested in this release.
- Physical access to a running machine, including bus, JTAG and cold-boot
  attacks.
- DMA from a device no VT-d unit in service covers: on a machine with AMD-Vi,
  which no build profile drives, with no remapping unit, or behind a unit that
  did not come up. The device is unconfined, and the serial log says so for each
  claim.
- DMA from a device found at boot that no driver has claimed, which stays in the
  [identity domain](glossary.md#identity-domain) and reaches all the memory the
  kernel manages, and from a device with no PCI requester id, which gets no
  domain (`pci_address` in `src/hardware/broker/confine/table.rs:35-43`).
- Interrupts a device raises by writing MSI messages. Interrupt remapping is off
  in every build profile (`nonos-iommu-intremap` in `Cargo.toml:879-884`).
- What sites learn from what the person sends them, everything under Direct, and
  each model download the person takes direct. Each of these shows this
  machine's address to the far end. A Linux package from a mirror on the local
  network is fetched directly too (`offline` in
  `userland/capsule_linux/src/linux/install/route_read.rs:84-94`).
- Seeing that NONOS runs Nym and Anyone. Both anonymity transports start on
  every boot that starts the network stack, whatever the choice (`spawn_nym` and
  `spawn_anon` in `src/userspace/init/spawn_plan/network/spawn.rs:17-29`).
  net.nym reaches for a gateway from its serve loop while idle (`_start` in
  `userland/capsule_net_nym/src/main.rs:48-63`), and net.anon fetches its
  directory, opens a link to a guard and builds a circuit while idle (`idle` in
  `userland/capsule_net_anon/src/server/idle.rs:43-51`).
- Serial output on an image built from any profile but `hardened` and
  `airgapped`. Capsules may write diagnostics to the [serial
  console](glossary.md#serial-console) there; only those two profiles take that
  out (`debugFeatures` in `tools/nix/config.nix:60-92`).
- A `dev` image, which promises nothing (`tools/nix/config.nix:102-109`).
- Timing and cache side channels between capsules, which can run at the same
  moment on cores that share a cache, since the kernel schedules on every core.
  At each syscall entry the kernel fills the return stack buffer and sets IBRS
  where the CPU has it (`kernel_entry` in
  `src/arch/x86_64/syscall/manager/entry.rs:34-36`). It also works out at boot
  what the CPU is exposed to and which mitigations are on, but writes that
  report through the structured log, which nothing initialises in this release,
  so it is never printed (`vulnerabilities` and `mitigations` in
  `src/security/hardening/spectre_mitigations/report.rs:22-44`). This model
  claims nothing beyond that.
- A kernel panic. It halts every CPU without the ZeroState wipe (`panic` in
  `src/boot/panic/handler.rs:41-64`).
