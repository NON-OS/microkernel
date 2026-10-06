# Memory and paging

Where things live in the NONOS kernel's virtual address space, how page tables are built and changed, which hardware protections the kernel turns on, and how it copies to and from user memory.

## Address layout

NONOS on x86_64 uses four-level paging with 48-bit canonical addresses. The user half ends at `CANONICAL_LOW_MAX` and the kernel half starts at `CANONICAL_HIGH_MIN` (`src/memory/layout/constants/canonical.rs:17-21`). The kernel is a static image linked with `-no-pie`, and the linker script places `__kernel_image_start` at `0xFFFFFFFF80000000` (`linker.ld:18-19`).

| Region | Constant | Start | Size |
|---|---|---|---|
| User space | `USER_TOP` (`src/memory/layout/constants/canonical.rs:21`) | `0x0` | 128 TiB |
| [Directmap](../overview/glossary.md#directmap) of physical memory | `DIRECTMAP_BASE` (`src/memory/layout/constants/regions.rs:30-31`) | `0xFFFF_8000_0000_0000` | 256 GiB |
| Kernel heap window, unused | `KHEAP_BASE` (`src/memory/layout/constants/regions.rs:33-34`) | `0xFFFF_FF00_0000_0000` | 256 MiB |
| Device registers | `MMIO_BASE` (`src/memory/layout/constants/mmio.rs:17-18`) | `0xFFFF_FF30_0000_0000` | 512 MiB |
| Kernel virtual ranges (vmap) | `VMAP_BASE` (`src/memory/layout/constants/mmio.rs:19-20`) | `0xFFFF_FF50_0000_0000` | 256 MiB |
| DMA buffers | `DMA_BASE` (`src/memory/layout/constants/mmio.rs:21-22`) | `0xFFFF_FF60_0000_0000` | 256 MiB |
| Secondary CPU stacks | `PERCPU_STACKS_BASE` (`src/memory/layout/constants/percpu.rs:19`) | `0xFFFF_FFD0_0000_0000` | 64 KiB per CPU |
| Kernel image | `KERNEL_BASE` (`src/memory/layout/constants/canonical.rs:19`) | `0xFFFF_FFFF_8000_0000` | text and data, 32 MiB windows each |

The device register window is where `map_device_memory` hands out uncached mappings, starting from `MMIO_BASE` (`src/memory/mmio/manager/core/types.rs:32`). The text and data window sizes are `KTEXT_SIZE` and `KDATA_SIZE` (`src/memory/layout/constants/sections.rs:19-22`).

A process sees a user half of its own and the shared kernel half. The ELF loader places a position-independent [capsule](../overview/glossary.md#capsule) image at `DEFAULT_PIE_BASE`, `0x40_0000`, plus a random page-aligned offset below `EXEC_RANDOMIZATION_RANGE`, 1 GiB, drawn by `randomize_base` for each load; an image that is not position independent loads at `0x40_0000` exactly (`src/elf/loader/core/loader/base_addr.rs:19-25`, `src/elf/aslr/manager/constants.rs:17`, `src/elf/aslr/manager/randomize.rs:24-30`). The 2 MiB user stack ends at `USER_STACK_BASE`, `0x0000_7FFF_FFFF_0000` (`src/process/userspace/constants.rs:30-32`), and `allocate_user_stack` leaves the page below it unmapped as a guard (`src/kernel_core/process_spawn/user_stack.rs:54-88`). Anonymous `MkMmap` ranges come from `USER_MMAP_BASE`, `0x8000_0000`, up to `0x7000_0000_0000` (`src/process/mmap_va.rs:36-37`). Process creation is on [processes and spawn](processes-and-spawn.md).
