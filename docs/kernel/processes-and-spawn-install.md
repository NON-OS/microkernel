# Processes and capsule spawn: install, the ELF loader and stacks

Part of [Processes and capsule spawn](processes-and-spawn.md).

## Install

`run` in `install` does the work in an order that leaves nothing behind on failure (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs`, `fn run`):

1. An empty ELF is refused, and so is a [reply inbox](../overview/glossary-peer-list.md#reply-inbox) name that `InboxName` cannot hold, before anything is registered (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs`, `fn run`).
2. The reply inbox and its endpoint are registered unowned, because the pid does not exist yet, through `register_or_get_bootstrap_inbox` (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs`, `fn run`).
3. `create_process_with_parent` makes the process in the `Ready` state with its scheduling band (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs`, `fn run`).
4. `finish` claims the reply endpoint, registers `proc.<pid>` (its requests and replies), `stdout.<pid>` (what it prints, for its parent to drain; never `proc.<pid>`, so a server never receives its own lines) and `stdin.<pid>` (`src/ipc/nonos_inbox/own_names.rs`), loads the ELF, installs the capabilities, allocates the kernel and user stacks, sets the first user context, registers the service endpoint and puts the process on the run queue (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs`, `fn finish`).
5. If any step after the pid exists fails, `teardown` ends the process with status -1, as an exit would (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs`, `fn run`).

`install_caps` trims the word through the boot profile, which removes `Network` on a boot that runs no network, then calls `install_spawn` once (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install_caps.rs`, `fn install_caps`). `for_capsule` starts 4 interactive, 7 network and 4 storage capsules in the `High` band and every other capsule in `Normal` (`src/kernel_core/process_spawn/capsule_spawn/runner/install/priority.rs`, `fn for_capsule`).

## Loading the ELF

`load_elf_entry_into` loads one image at a time under a global loader lock and returns the entry point (`src/elf/loader/global.rs`, `fn load_elf_entry_into`). The steps are in `load_entry_into` (`src/elf/loader/core/loader/load_entry_into.rs`, `fn load_entry_into`):

- `validate_elf` accepts only a 64 bit, little endian, current version ELF with the native header sizes, the machine `EM_NATIVE` and the type `ET_EXEC` or `ET_DYN` (`src/elf/loader/core/parse_header/validate.rs`, `fn validate_elf`). On x86_64 `EM_NATIVE` is `EM_X86_64` (`src/elf/types/constants/machine.rs`, `const EM_X86_64`).
- Capsules are built position independent, as the `position-independent-executables` key of [userland/x86_64-nonos-user.json](../../userland/x86_64-nonos-user.json) sets. `load_base` places such an image at `DEFAULT_PIE_BASE` plus a random page aligned offset below `EXEC_RANDOMIZATION_RANGE`, from `randomize_base` (`src/elf/aslr/manager/randomize.rs`, `fn randomize_base`). `DEFAULT_PIE_BASE` is `0x400000` (`src/elf/loader/core/loader/state.rs`, `const DEFAULT_PIE_BASE`) and `EXEC_RANDOMIZATION_RANGE` is `0x4000_0000` (`src/elf/aslr/manager/constants.rs`, `const EXEC_RANDOMIZATION_RANGE`). The offset comes from `RDRAND` on x86_64, or from a fixed linear congruential step when the CPU gives no random value, in `random_offset` (`src/elf/aslr/manager/entropy.rs`, `fn random_offset`).
- Each `PT_LOAD` segment must have its file size within its memory size, must not be both writable and executable (`WXViolation`), must be aligned as declared and must lie inside the file (`src/elf/loader/core/load_segment/validate.rs`, `fn input_fields`).
- Pages are user readable, writable only for a writable segment and executable only for an executable one, as `pte_perms_from_phdr` sets them (`src/elf/loader/core/load_segment/pte_flags.rs`, `fn pte_perms_from_phdr`).
- Relative relocations are applied, then `enforce_relro` makes the `PT_GNU_RELRO` span read only, so a capsule cannot rewrite its own GOT (`src/elf/loader/core/relro.rs`, `fn enforce_relro`).

## Stacks

The user stack is `USER_STACK_SIZE`, 2 MiB, ending at `USER_STACK_BASE`, `0x0000_7FFF_FFFF_0000`, and each process has a kernel stack of `KERNEL_STACK_SIZE`, 32 KiB (`src/process/userspace/constants.rs`, `const KERNEL_STACK_SIZE`). The page below the user stack is left unmapped, so an overflow faults as a user fault, and stack pages are never executable, as `allocate_user_stack` maps them (`src/kernel_core/process_spawn/user_stack.rs`, `fn allocate_user_stack`). The kernel stack comes from the kernel's virtual range allocator, described on [Frame allocator](frame-allocator.md#kernel-virtual-ranges).
