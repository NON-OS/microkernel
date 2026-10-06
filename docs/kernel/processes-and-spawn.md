# Processes and capsule spawn

How the NONOS kernel turns a signed [capsule](../overview/glossary.md#capsule) into a running process: the gate it must pass, the ELF loader, the address space and stacks, and what happens when the process ends.

## Processes and address spaces

Every ring 3 program is a process with its own address space. `create_address_space` gives each one a fresh top level page table and copies the kernel half into it (`src/memory/paging/manager/address_space/create.rs:39-48`), and `allocate` records that table for the process (`src/process/address_space/lifecycle/setup.rs:26-49`). A thread shares its process's tables through `inherit` (`src/process/address_space/lifecycle/setup.rs:76-78`) and is held to its process's name and limits when it sends IPC, through `caller_group` (`src/services/registry/peers_check.rs:23-36`).

A process moves through the states of `ProcessState`: `New`, `Ready`, `Running`, `Sleeping`, `Stopped`, then `Zombie` and `Terminated` with an exit code (`src/process/core/types.rs:26-33`). [Memory and paging](memory-and-paging.md) and [Scheduler and SMP](scheduler-and-smp.md) cover the tables and the run queues.

## Where capsules come from

- Built into the image. The kernel embeds a capsule's ELF, identity certificate, [manifest](../overview/glossary.md#manifest) and [attestation trailer](../overview/glossary.md#attestation-trailer) with `include_bytes!`, as `HELLO_ELF` and its neighbours do (`src/userspace/capsule_hello/embed.rs:17-35`). A spawn site gives the name, the service and reply ports and the [capabilities](../overview/glossary.md#capability) it is willing to grant, as `spawn_hello_capsule` does for `app.hello` on port 4810 with reply port 4811 (`src/userspace/capsule_hello/spawn.rs:28-58`). Its `HELLO_ELF` is embedded only in a kernel built with the `nonos-capsule-hello` feature (`src/userspace/capsule_hello/embed.rs:17-23`).
- Installed later. The installer reads the same four artifacts from the store and passes them to `MkCapsuleLoad` by pointer; `load_capsule_from_vfs` takes the service name and both [endpoints](../overview/glossary.md#endpoint) from the signed manifest and limits the requested bits to what the manifest declares (`src/kernel_core/process_spawn/capsule_spawn/from_vfs/load/spawn.rs:34-68`). The call needs `CoreExec`, `IPC` and `Memory` (`src/syscall/contract/cap_table/mk.rs:85-88`).
- On behalf of another process. A caller holding `SpawnBroker` may name a live pid as the new capsule's parent; `attest` is the only way to build that attribution (`src/kernel_core/process_spawn/capsule_spawn/attested_parent.rs:36-46`).
- Linux guests. `MkForeignSpawn` creates a process for the [Linux personality](../overview/glossary.md#linux-personality) that holds no capabilities at all: `install_spawn` gives it an empty word (`src/process/foreign/spawn.rs:65-68`); its system calls go to the supervising capsule. [Linux personality](../userland/linux-personality.md) covers it.

## The spawn gate

Every verified capsule, built in or installed, goes through `spawn_verified_as`, and any refusal comes back as a `SpawnError` (`src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs:38-82`).

```mermaid
flowchart TD
    P[profile_gate] -->|refused| X[SpawnError]
    P --> F[preflight]
    F -->|certificate or manifest refused| X
    F --> G[attest_gate]
    G -->|no trailer or proof refused| X
    G --> I[install]
    I -->|any step fails| T[teardown]
    I --> R[record_attested]
```

1. The [boot profile](../overview/glossary.md#boot-profile) may refuse the capsule by name; `check` in `profile_gate` prints a `[PROFILE]` line and returns `ProfileRefused` (`src/kernel_core/process_spawn/capsule_spawn/runner/profile_gate.rs:31-46`).
2. `run` in `preflight` decodes the identity certificate with `decode_id_cert` and checks it against the baked trust anchor with `verify_id_cert` (`src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:41-44`).
3. `verify_with_publisher` checks the manifest and its publisher signature against the ELF, the target triple, the requested capabilities and the two declared endpoints, and returns the capabilities to install (`src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:46-67`). The manifest may not ask for any bit above the `allowed_caps_ceiling` of the identity certificate, which `check_ceiling` enforces (`src/security/capsule_manifest/verify/caps.rs:21-29`).
4. `classify` puts a capsule in the `systems.nonos` namespace in the `Enrolled` tier and any other in the `Publisher` tier (`src/kernel_core/process_spawn/capsule_spawn/runner/tier.rs:22-28`). Both tiers end in the same `attest_gate` (`src/kernel_core/process_spawn/capsule_spawn/runner/publisher_gate.rs:28-33`), which `run` calls with the manifest's `required_caps` (`src/kernel_core/process_spawn/capsule_spawn/runner/preflight.rs:70-77`).
5. `attest_gate` refuses a capsule with no trailer and otherwise calls `verify_capsule_attestation`, printing a `[ZK-ATTEST]` line either way (`src/kernel_core/process_spawn/capsule_spawn/runner/attest_gate.rs:23-63`).
6. `install` creates the process (below).
7. `record_attested` enters the pid, the proved measurement, the [capability word](../overview/glossary.md#capability-word) and the authority that vouched in the attestation registry (`src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs:64-80`).

What the attestation check does, from the kernel side: `measure` takes the BLAKE3 digest of the ELF (`src/security/capsule_attest/measure.rs:26-32`), and `verify_capsule_attestation` tries the vendor's policy root first and then each root a person enrolled on the machine (`src/security/capsule_attest/verify.rs:33-63`). Against a root, `verify_against` builds the leaf from that digest, the capability word it is given and the policy epoch, folds the trailer's path to the kernel's own root, and checks the STARK proof over the same statement (`src/security/capsule_attest/path.rs:35-54`). The word it is given is the manifest's required capabilities; the optional bits a spawn site adds are bounded by the signed manifest and the certificate ceiling, not by the proof. A capsule whose bytes or required capabilities differ from what was proved is refused. Against an enrolled root, a trailer that starts with the `local_build` magic, for a capsule this machine built itself, carries a keyed tag only this kernel can mint or check, and `local_build::verify` checks that tag instead of a STARK proof (`src/security/capsule_attest/against_root.rs:34-45`). The proof system itself is on [STARK attestation](../security/stark-attestation.md), and signatures on [Signing and publisher keys](../userland/signing-and-publisher-keys.md).

A development image may admit capsules on the path alone with `nonos-dev-attest`; `compile_error` refuses that feature together with `nonos-release` (`src/lib.rs:43-47`).

The spawn site does not choose the capabilities. The word installed comes from the verified manifest, and `requested_caps` is only an upper bound for optional bits (`src/kernel_core/process_spawn/capsule_spawn/runner/verified.rs:25-27`).

## Install

`run` in `install` does the work in an order that leaves nothing behind on failure (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs:28-77`):

1. An empty ELF is refused, and so is a [reply inbox](../overview/glossary.md#reply-inbox) name that `InboxName` cannot hold, before anything is registered (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs:30-40`).
2. The reply inbox and its endpoint are registered unowned, because the pid does not exist yet, through `register_or_get_bootstrap_inbox` (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs:41-43`).
3. `create_process_with_parent` makes the process in the `Ready` state with its scheduling band (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs:44-56`).
4. `finish` claims the reply endpoint, registers `proc.<pid>` and `stdin.<pid>`, loads the ELF, installs the capabilities, allocates the kernel and user stacks, sets the first user context, registers the service endpoint and puts the process on the run queue (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs:88-117`).
5. If any step after the pid exists fails, `teardown` ends the process with status -1, as an exit would (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs:70-80`).

`install_caps` trims the word through the boot profile, which removes `Network` on a boot that runs no network, then calls `install_spawn` once (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install_caps.rs:20-24`). `for_capsule` starts 4 interactive, 7 network and 4 storage capsules in the `High` band and every other capsule in `Normal` (`src/kernel_core/process_spawn/capsule_spawn/runner/install/priority.rs:64-70`).

## Loading the ELF

`load_elf_entry_into` loads one image at a time under a global loader lock and returns the entry point (`src/elf/loader/global.rs:50-54`). The steps are in `load_entry_into` (`src/elf/loader/core/loader/load_entry_into.rs:20-44`):

- `validate_elf` accepts only a 64 bit, little endian, current version ELF with the native header sizes, the machine `EM_NATIVE` and the type `ET_EXEC` or `ET_DYN` (`src/elf/loader/core/parse_header/validate.rs:17-49`). On x86_64 `EM_NATIVE` is `EM_X86_64` (`src/elf/types/constants/machine.rs:29`).
- Capsules are built position independent, as the `position-independent-executables` key of [userland/x86_64-nonos-user.json](../../userland/x86_64-nonos-user.json) sets. `load_base` places such an image at `DEFAULT_PIE_BASE` plus a random page aligned offset below `EXEC_RANDOMIZATION_RANGE`, from `randomize_base` (`src/elf/aslr/manager/randomize.rs:24-30`). `DEFAULT_PIE_BASE` is `0x400000` (`src/elf/loader/core/loader/state.rs:21-22`) and `EXEC_RANDOMIZATION_RANGE` is `0x4000_0000` (`src/elf/aslr/manager/constants.rs:17`). The offset comes from `RDRAND` on x86_64, or from a fixed linear congruential step when the CPU gives no random value, in `random_offset` (`src/elf/aslr/manager/entropy.rs:28-39`).
- Each `PT_LOAD` segment must have its file size within its memory size, must not be both writable and executable (`WXViolation`), must be aligned as declared and must lie inside the file (`src/elf/loader/core/load_segment/validate.rs:20-47`).
- Pages are user readable, writable only for a writable segment and executable only for an executable one, as `pte_perms_from_phdr` sets them (`src/elf/loader/core/load_segment/pte_flags.rs:20-29`).
- Relative relocations are applied, then `enforce_relro` makes the `PT_GNU_RELRO` span read only, so a capsule cannot rewrite its own GOT (`src/elf/loader/core/relro.rs:27-57`).

## Stacks

The user stack is `USER_STACK_SIZE`, 2 MiB, ending at `USER_STACK_BASE`, `0x0000_7FFF_FFFF_0000`, and each process has a kernel stack of `KERNEL_STACK_SIZE`, 32 KiB (`src/process/userspace/constants.rs:30-32`). The page below the user stack is left unmapped, so an overflow faults as a user fault, and stack pages are never executable, as `allocate_user_stack` maps them (`src/kernel_core/process_spawn/user_stack.rs:26-57`).

## Exit and reaping

```mermaid
stateDiagram-v2
    [*] --> Ready
    Ready --> Running
    Running --> Sleeping
    Sleeping --> Ready
    Running --> Zombie
    Sleeping --> Zombie
    Zombie --> Terminated
    Terminated --> [*]
```

A process ends in one of three ways, all through `exit_and_yield` or `teardown`: it calls `MkExit` (`src/syscall/microkernel/process.rs:35-52`), a fault in ring 3 or a signal kills it (`src/process/exit/exit_and_yield.rs:17-38`), or another process ends it with `MkKill`.

`teardown` runs at once (`src/process/exit/teardown.rs:21-89`):

- It releases the process's surfaces, then every MMIO, IRQ, DMA and PIO grant it holds in the [hardware broker](hardware-broker.md), in `release_all_for_pid` and its neighbours (`src/process/exit/teardown.rs:44-51`). The MMIO release also drops the process's device claims: `release_all_for_pid` in the claim module stops bus mastering on each device and detaches it from the capsule's IOMMU domain, before the DMA buffers are freed (`src/hardware/broker/claim/release.rs:51-62`).
- It drops the calls the process made and the replies it owed, with `release_pending_replies_for_pid` (`src/process/exit/teardown.rs:52`).
- It marks the process `Zombie` and releases its names, its endpoints and its reply inbox, with `release_names` (`src/process/exit/teardown.rs:54-57`), so a relaunch does not collide with the dead process's names.
- It records the exit code for the parent with `reap_log::record` and queues the process for finalization (`src/process/exit/teardown.rs:64-82`).

`finalize_teardown` runs later, from a timer tick, once `drain` finds no CPU still using the process's page tables (`src/process/exit/pending.rs:27-61`). It frees the address space, drops the [inboxes](../overview/glossary.md#inbox) or keeps the output inbox for the parent, reparents orphans with `reparent_orphans` and removes the process row (`src/process/exit/finalize.rs:11-39`). Removing the row sets the state to `Terminated` first, in `terminate_process` (`src/process/core/table/ops.rs:22-31`).

`MkWait(pid, timeout_ms)` is for the parent only. `sys_wait` returns the child's exit code, `ECHILD` for a pid that is not the caller's child, or `ETIMEDOUT` when the deadline passes; it checks every `SLICE_MS`, 5 ms, and a `timeout_ms` of 0 checks once and returns (`src/syscall/microkernel/wait.rs:21-51`). After the row is gone the code is read from the reap log, which keeps 64 entries and drops the lowest pid first, as `record` does (`src/process/exit/reap_log.rs:24-36`).

`MkKill(pid, sig)` accepts only `SIGINT`, `SIGTERM` and `SIGKILL`. `sys_kill` lets a parent end its child, a supervisor end its guest, and a holder of `ProcessControl` or `Admin` end anything, and gives the exit code 128 plus the signal (`src/syscall/microkernel/kill.rs:26-62`). The signal numbers are 2, 15 and 9: `SIGINT` (`src/process/signal/constants.rs:18`), `SIGTERM` (`src/process/signal/constants.rs:31`) and `SIGKILL` (`src/process/signal/constants.rs:25`).

A capsule whose name starts with `driver.` and that exits on its own with a status other than 0 gets a serial line from the kernel, as `told` decides: status 2 is read as no device present and 6 as a device that never came up, in `words` (`src/process/exit/end_rule.rs:24-44`).
