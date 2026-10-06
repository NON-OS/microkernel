# Boot handoff and kernel init

How control passes from the NONOS bootloader to the x86_64 kernel, what the kernel checks before it trusts what it was given, and the order it brings itself up in until the first [capsule](../overview/glossary.md#capsule) runs.

## The last steps of the bootloader

The loader's `exit_and_jump` does the transfer. While UEFI boot services can still hand out page-table frames, it builds the kernel's page tables with `build_kernel_pml4` (`nonos-bootloader/src/handoff/exit/orchestrate.rs:72`). Those tables hold an identity map of low physical memory and of the framebuffer, a linear map of physical memory at `PML4[256]` (the [directmap](../overview/glossary.md#directmap)), and the kernel image in the upper half, in the order `build_kernel_pml4` adds them (`nonos-bootloader/src/paging/build.rs:45-62`). The identity window is `IDENTITY_LOW_BYTES`, 64 GiB (`nonos-bootloader/src/paging/constants.rs:43`).

The boot stack and the handoff structure are passed as directmap addresses, not physical ones, through `phys_to_directmap_virt` (`nonos-bootloader/src/handoff/exit/orchestrate.rs:86-93`). The kernel later removes the identity window, and a directmap address stays valid on both sides of that change.

The loader then calls `exit_boot_services`, copies the final UEFI memory map with `copy_memory_map` and records it in the handoff (`nonos-bootloader/src/handoff/exit/orchestrate.rs:102-106`). It loads the new page tables with `switch_to_kernel_pml4` (`nonos-bootloader/src/handoff/exit/orchestrate.rs:115`). Before the jump, `validate_and_jump` checks the entry, stack and handoff addresses (`nonos-bootloader/src/handoff/exit/validate.rs:28-46`). `validate_handoff_address` refuses one that is zero, below 1 MiB, not 8-byte aligned or not canonical (`nonos-bootloader/src/handoff/jump/validate.rs:47-55`), and the loader then halts with code `0xE3` instead of jumping (`nonos-bootloader/src/handoff/exit/validate.rs:38-41`).

The jump itself is `nonos_arch_handoff_jump` (`nonos-bootloader/src/arch/x86_64/asm/handoff_jump.S:33-53`). It masks interrupts, loads the stack pointer, moves the handoff address into `rdi` and the entry address into `rax`, clears every other general register and jumps through `rax`. There is no way back to the loader.
