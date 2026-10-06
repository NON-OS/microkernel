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

## How the page tables come to be

The bootloader builds the first tables. The directmap maps the first 256 GiB of physical memory with `PTE_NX` set, in 1 GiB leaves where the CPU has them and 2 MiB leaves otherwise (`nonos-bootloader/src/paging/map_directmap.rs:24-37`). The kernel reaches page tables, user frames and the handoff through it. See [boot handoff](boot-handoff.md) for the identity window the loader adds.

`init_unified_vm` takes over from there (`src/memory/unified/init/run.rs:35-120`). In order, it:

1. starts the paging manager on the live CR3;
2. refuses to go on if the kernel half of the PML4, entries 256 to 511, is empty;
3. allocates and frees one frame to prove the frame allocator works;
4. starts the page allocator for kernel virtual ranges;
5. maps the local APIC registers uncached in the kernel half, so they stay reachable;
6. once the kernel half holds at least the directmap and the kernel image, calls `clear_low_half`, which zeroes PML4 entries 0 to 255 and reloads CR3 (`src/arch/x86_64/paging/clear_low_half.rs:42-58`).

From then on the kernel runs only from the upper half. Every new address space copies entries 256 to 511 from the kernel's table with `clone_kernel_half_into` (`src/memory/paging/manager/address_space/clone.rs:35-50`), so no process inherits a low mapping by accident. A failure in any step is a [boot stop](../overview/glossary.md#boot-stop) with `memory: init_unified_vm failed`.

Pages are 4 KiB, with `HUGE_PAGE_2M` and `HUGE_PAGE_1G` leaves possible (`src/memory/layout/constants/page.rs:17-21`). One low mapping comes back for a while: the AP trampoline. `install` maps the 16 pages at physical `0x8000` read and execute while the other CPUs start, and `remove` unmaps them and clears the low half again (`src/smp/init/ap_identity.rs:40-71`). See [scheduler and SMP](scheduler-and-smp.md).

## W^X

No mapping may be writable and executable at once. `map_page` returns `WXViolation` for such a request (`src/memory/paging/manager/mapping/map.rs:41-43`), and a protection change is checked the same way with `is_wx_violation` (`src/memory/paging/manager/protection/update.rs:33-34`).

The kernel image has three load segments in the linker script's `PHDRS`: text read and execute, read-only data, and data read and write (`linker.ld:11-15`). `report_kernel_sections` compares the live mappings with the four entries `kernel_sections` returns, text, read-only data, data and bss (`src/memory/layout/manager/state.rs:49-80`), and prints `[KSEC] 4/4 sections mapped as declared`, or names each page that differs and ends with `WARNING W^X not held` (`src/kernel_core/init/entry/report_sections.rs:33-51`). It reports and does not stop the boot.

## CPU protections

`init_vm_and_protection` turns the ring 0 restrictions on once the final tables exist. `apply` refuses a CPU without execute-never, then enables SMEP, SMAP and UMIP where CPUID reports them, sets EFER.NXE and CR0.WP, and reads every one back (`src/memory/mmu/mmu/protect/apply.rs:26-44`). On a CPU without NX, `init_mmu` fails and the boot stops with `memory: init_mmu failed` (`src/kernel_core/init/entry/init_vm_and_protection.rs:41-43`). With SMAP live, `clear_alignment_check` clears EFLAGS.AC, since SMAP is not enforced while AC is set (`src/memory/mmu/mmu/protect/cr4.rs:46-49`).

`report` prints the result as one line on the [serial console](../overview/glossary.md#serial-console) (`src/memory/mmu/mmu/protect/report.rs:25-43`):

```text
[CPU-PROT] smep=1 smap=1 umip=1 nx=1 wp=1
```

Any zero other than UMIP adds `[CPU-PROT] WARNING kernel is not fully protected from user pages`. Each secondary CPU applies the same bits itself, and `finish` keeps it from running user code if it ended up weaker than the boot CPU (`src/smp/ap/user_setup.rs:46-59`).

On aarch64 the same properties come from the PXN, UXN and AP bits in each descriptor. PAN is not enabled, and `report_el1_protection` says so on the console (`src/kernel_core/init/entry/init_vm_and_protection.rs:84-89`).

## Stack guards

Each fault stack of the boot CPU is a `GuardedStack` with a 4096-byte `GUARD_BYTES` page below it (`src/arch/x86_64/gdt/guarded_stack.rs:27-34`). Once paging is up, `arm_stack_guards` unmaps those pages and prints `[STACK-GUARD] bsp armed n/m`, with a warning when not every guard was taken out (`src/kernel_core/init/entry/init_vm_and_protection.rs:59-73`). Each secondary CPU arms its own with `arm_ap_guards` and prints no count (`src/smp/ap/bring_up.rs:55-56`).

The 64 KiB kernel stacks of secondary CPUs are mapped back to back by `allocate`, with no unmapped page between them (`src/smp/init/stack.rs:22-32`). An overflow there runs into the next CPU's stack.
