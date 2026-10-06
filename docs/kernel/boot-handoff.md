# Boot handoff and kernel init

How control passes from the NONOS bootloader to the x86_64 kernel, what the kernel checks before it trusts what it was given, and the order it brings itself up in until the first [capsule](../overview/glossary.md#capsule) runs.

## The last steps of the bootloader

The loader's `exit_and_jump` does the transfer. While UEFI boot services can still hand out page-table frames, it builds the kernel's page tables with `build_kernel_pml4` (`nonos-bootloader/src/handoff/exit/orchestrate.rs:72`). Those tables hold an identity map of low physical memory and of the framebuffer, a linear map of physical memory at `PML4[256]` (the [directmap](../overview/glossary.md#directmap)), and the kernel image in the upper half, in the order `build_kernel_pml4` adds them (`nonos-bootloader/src/paging/build.rs:45-62`). The identity window is `IDENTITY_LOW_BYTES`, 64 GiB (`nonos-bootloader/src/paging/constants.rs:43`).

The boot stack and the handoff structure are passed as directmap addresses, not physical ones, through `phys_to_directmap_virt` (`nonos-bootloader/src/handoff/exit/orchestrate.rs:86-93`). The kernel later removes the identity window, and a directmap address stays valid on both sides of that change.

The loader then calls `exit_boot_services`, copies the final UEFI memory map with `copy_memory_map` and records it in the handoff (`nonos-bootloader/src/handoff/exit/orchestrate.rs:102-106`). It loads the new page tables with `switch_to_kernel_pml4` (`nonos-bootloader/src/handoff/exit/orchestrate.rs:115`). Before the jump, `validate_and_jump` checks the entry, stack and handoff addresses (`nonos-bootloader/src/handoff/exit/validate.rs:28-46`). `validate_handoff_address` refuses one that is zero, below 1 MiB, not 8-byte aligned or not canonical (`nonos-bootloader/src/handoff/jump/validate.rs:47-55`), and the loader then halts with code `0xE3` instead of jumping (`nonos-bootloader/src/handoff/exit/validate.rs:38-41`).

The jump itself is `nonos_arch_handoff_jump` (`nonos-bootloader/src/arch/x86_64/asm/handoff_jump.S:33-53`). It masks interrupts, loads the stack pointer, moves the handoff address into `rdi` and the entry address into `rax`, clears every other general register and jumps through `rax`. There is no way back to the loader.

## The handoff structure

The [boot handoff](../overview/glossary.md#boot-handoff) is one `#[repr(C)]` structure, `BootHandoffV1`, defined identically on both sides (`src/boot/handoff/types/handoff.rs:28-46`, `nonos-bootloader/src/handoff/types/handoff.rs:28-47`). Its first fields are `magic`, `version` and `size`. The constants file holds the magic value `0x4E4F4E4F`, the layout version 2 and `MAX_CMDLINE_LEN` (`src/boot/handoff/types/constants.rs:17-19`).

| Field | What it carries |
|---|---|
| `magic`, `version`, `size` | the layout's identity; the kernel compares all three |
| `flags` | the bits in the next table |
| `entry_point` | the kernel entry address the loader jumped to |
| `fb` | the UEFI GOP framebuffer: base, size, width, height, stride in bytes, pixel format, panel size in millimetres |
| `mmap` | pointer, entry size and count of the UEFI memory map copy |
| `acpi`, `smbios` | the ACPI RSDP and the SMBIOS entry point |
| `modules` | pointer and count of loaded modules |
| `timing` | the loader's TSC rate estimate and the UEFI time in Unix milliseconds |
| `meas`, `zk`, `policy` | the boot measurements and verification results the loader recorded |
| `rng` | a 32-byte seed for the kernel random generator |
| `firmware` | device firmware blobs read from the boot medium |
| `cmdline_ptr` | an optional kernel command line |

The `flags` word uses these bits, as listed in the `flags` module (`src/boot/handoff/types/constants.rs:33-54`):

| Bit | Name | Meaning |
|---|---|---|
| 0, 1 | `WX`, `NXE` | always set by the loader |
| 2 to 4 | `SMEP`, `SMAP`, `UMIP` | the CPU has this protection, as the loader detected it |
| 5 | `IDMAP_PRESERVED` | always set: the low identity map is present at entry |
| 6 | `FB_AVAILABLE` | `fb` is valid |
| 7 | `ACPI_AVAILABLE` | `acpi` is valid |
| 8 | `TPM_MEASURED` | the loader extended TPM measurements |
| 9 | `SECURE_BOOT` | the loader reports UEFI Secure Boot as on |
| 10 | `ZK_ATTESTED` | the loader reports the attestation proof as verified |
| 11 | `INSTALL_REQUESTED` | the person chose the boot menu's install entry |
| 12 to 15 | `PROFILE_HARDENED`, `PROFILE_SAFE`, `PROFILE_AIR_GAPPED`, `PROFILE_RECOVERY` | the [boot profile](../overview/glossary.md#boot-profile); Standard sets none |

The loader's `build_handoff_flags` sets the framebuffer, ACPI, Secure Boot, attestation, TPM, SMEP, SMAP and UMIP bits from what it found, and sets `WX`, `NXE` and `IDMAP_PRESERVED` on every boot (`nonos-bootloader/src/handoff/prepare/flags.rs:20-57`). The install bit and the profile bits come from the boot menu: `handoff_flag` adds `INSTALL_REQUESTED` (`nonos-bootloader/src/handoff/types/install.rs:45-46`), and the chosen mode adds its `PROFILE_*` bit (`nonos-bootloader/src/menu/types/mode.rs:61-64`).

A few sizes are fixed by the code:

- Each memory map entry is a `MemoryMapEntry`: type, padding, physical start, virtual start, page count and attributes, 40 bytes in all (`src/boot/handoff/types/memory.rs:42-49`). Only entries of type `CONVENTIONAL`, 7, count as usable RAM (`src/boot/handoff/types/memory.rs:25`).
- The `AttestPolicy` block is asserted at build time to be 176 bytes (`nonos-bootloader/src/handoff/types/security.rs:68`).
- The firmware table holds at most `MAX_FIRMWARE_ENTRIES`, 64 (`src/boot/handoff/types/firmware.rs:17`).
- The command line is read up to `MAX_CMDLINE_LEN`, 4096 bytes (`src/boot/handoff/types/constants.rs:19`). The `cmdline` reader returns nothing for a pointer above 48 bits or for a control byte other than tab, line feed or carriage return (`src/boot/handoff/types/handoff.rs:79-112`).

## What the kernel checks

`init_handoff` accepts the handoff once and checks it in this order (`src/boot/handoff/api/init.rs:40-72`):

1. The pointer is non-zero, 8-byte aligned and canonical.
2. `magic`, `version` and `size` match this kernel's `BootHandoffV1` exactly.
3. The framebuffer, memory map and RSDP pointers fit in 48 bits, the `MAX_PHYS_PTR` limit (`src/boot/handoff/api/init.rs:28`).
4. `validate_security` runs four checks (`src/boot/handoff/api/security/orchestrator.rs:21-27`). The random seed must not be all zero, or the result is `WeakEntropy` (`src/boot/handoff/api/security/entropy.rs:20-25`). When a memory map is present, its entry size must equal the kernel's own, or the result is `MemoryMapEntrySize` (`src/boot/handoff/api/security/memory_map.rs:26-28`). When `FB_AVAILABLE` is set, the framebuffer needs a non-zero width, height and stride, a stride of at least one row of pixels, and a frame that fits its size, or the result is `FramebufferGeometry` (`src/boot/handoff/api/security/framebuffer.rs:20-47`). The entry point must lie inside the 256 MiB `KERNEL_IMAGE_WINDOW` above `KERNEL_BASE`, or in the low half above 1 MiB (`src/boot/handoff/api/security/entry_point.rs:24-36`).
5. A second call fails with `AlreadyInitialized`.

Each failure is one `HandoffError` with a fixed text, such as `Invalid handoff magic value` or `Bootloader entropy seed is all zero` (`src/boot/handoff/api/error/handoff_error.rs:20-47`). A version or size mismatch is a refusal, so a loader and a kernel built for different handoff versions do not boot together.

On failure `kernel_entry` prints `[NONOS] Handoff FAIL` and `[NONOS] Handoff ERR:` with the error text on the [serial console](../overview/glossary.md#serial-console), then calls `vga_fallback` (`src/nonos_main.rs:79-88`). `vga_fallback` writes `NONOS <version> <channel> - No framebuffer available` into the legacy VGA text buffer at `0xB8000` and halts (`src/entry/fallback.rs:14-43`). The panic handler's comment says the VGA text buffer is invisible on UEFI machines, which is why the handler also calls `panic_screen` (`src/boot/panic/handler.rs:54-57`). A refused handoff has no such step: its only mark on the framebuffer is the breadcrumb segment described below. What a given panel shows here was not tested on hardware in this release. See [panic and boot stop](panic-and-boot-stop.md).
