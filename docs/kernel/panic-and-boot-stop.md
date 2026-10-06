# Panic and boot stop

What the NONOS kernel does when it cannot go on, what the panel and the [serial console](../overview/glossary.md#serial-console) show in each case, and what a person can do next.

## At a glance

| Event | Panel | Serial console | Other CPUs |
|---|---|---|---|
| A boot step fails | notice band `NONOS BOOT STOPPED` | `[FATAL]` line | not started yet, or idle |
| Install asked, no installer | notice band | `[ERROR]` line | keep idling |
| Bootloader not admitted | notice band | `[ERROR]` line | keep idling |
| Handoff refused | breadcrumb strip, VGA text | `[NONOS] Handoff FAIL` | not started yet |
| Kernel panic | red band `KERNEL PANIC` | panic message | stopped by NMI |
| Kernel CPU exception | nothing new | `[PANIC xx]`, after `[TRAP xx]` for some | not signalled |
| Fault in a [capsule](../overview/glossary.md#capsule) | nothing | `[TRAP xx]` for `PF`, `GP`, `UD` | unaffected; the capsule ends |
| Kernel heap exhausted | VGA text | `[OOM]` lines | not signalled |
| [TLB shootdown](../overview/glossary.md#tlb-shootdown) timeout | nothing new | `[FATAL]` line | stopped by NMI |

Every stop ends in a loop that masks interrupts and halts the CPU, as `halt` does (`src/arch/x86_64/abi.rs:30-36`). NONOS does not restart the machine on its own. Nothing in the kernel arms or disarms the chipset watchdog that `detect_tco_watchdog` could find (`src/arch/x86_64/watchdog/mod.rs:24-25`), so a watchdog that firmware left running is not the kernel's to stop. Whether any firmware does that was not tested in this release.

## A boot step fails

A [boot stop](../overview/glossary.md#boot-stop) goes through `stop` (`src/boot/stop.rs:25-42`). It prints `[FATAL] <step>: <detail>` on the serial console, draws a notice band titled `NONOS BOOT STOPPED` with the step, the detail and a hint line pointing to the serial log, and halts. Stages of kernel init call it through `fatal`, which first prints the step as an `[ERROR]` line (`src/kernel_core/init/entry/fatal.rs:19-22`).

These are the steps that stop the boot this way, with the step text the `[FATAL]` line and the band show:

| Step text | Where |
|---|---|
| `arch GDT init failed` | `init_cpu_tables` (`src/boot/main/core_init/cpu_tables.rs:28`) |
| `arch syscall init failed` | `syscall::init` in `init_cpu_tables` (`src/boot/main/core_init/cpu_tables.rs:32`) |
| `preemption timer install failed` | `install_on_bsp` (`src/boot/main/core_init/init_core_systems.rs:43`) |
| `security: speculation mitigations failed` | `speculation::init` (`src/kernel_core/init/entry/init_core_services.rs:31`) |
| `crypto: init_rng failed` | `init_rng` (`src/kernel_core/init/entry/init_core_services.rs:35`) |
| `ipc: init_ipc_secret failed` | `init_ipc_secret` (`src/kernel_core/init/entry/init_core_services.rs:38`) |
| `smp: init_bsp failed` | `init_bsp` (`src/kernel_core/init/entry/init_core_services.rs:41`) |
| `memory: init_unified_vm failed` | `init_unified_vm` (`src/kernel_core/init/entry/init_vm_and_protection.rs:29`) |
| `memory: init_mmu failed` | `init_mmu`, a CPU without NX (`src/kernel_core/init/entry/init_vm_and_protection.rs:42`) |
| `memory: protection flags unreadable` | `protection_flags` (`src/kernel_core/init/entry/init_vm_and_protection.rs:47`) |
| `cpu: SSE bring-up failed` | `enable_sse_avx` (`src/kernel_core/init/entry/init_extended_state.rs:31`) |
| `Init process creation failed` | `boot::stop` after `create_process` (`src/kernel_core/init/entry/microkernel_main.rs:46`) |
| `Init address space creation failed` | `create_address_space` (`src/kernel_core/init/entry/microkernel_main.rs:52`) |
| `Init kernel stack allocation failed` | `allocate_kernel_stack` (`src/kernel_core/init/entry/microkernel_main.rs:56`) |

Three of these show no band. `memory: init_mmu failed`, `memory: protection flags unreadable` and `cpu: SSE bring-up failed` happen after the low identity map is gone and before the kernel maps the framebuffer, so they reach the serial console only; see the next section.

One more step stops without a band. If the hardware reports memory encryption and turning it on fails, `init_memory_encryption` prints `[FATAL] memory encryption: hardware fault during enable` and halts, rather than run in plaintext on a machine that claims otherwise (`src/boot/main/init_memory_encryption.rs:22-50`).

`stop` halts only the CPU that calls it. The last three steps run after the other CPUs have started; they stay in their idle loops with nothing to run.

## Which screen the band is drawn on

`screen` picks the framebuffer: the kernel's own mapping once `init_arch_framebuffer` has made it, and before that the loader's identity mapping of the firmware framebuffer, while that mapping still exists (`src/sys/boot_log/screen.rs:26-54`). `show_notice` then fills a band across the top and writes the lines, without allocating and without taking a lock (`src/sys/boot_log/notice_screen.rs:34-47`).

Two cases show nothing on the panel. A stop between the removal of the low identity map in `init_vm_and_protection` and the framebuffer mapping later in kernel init has no framebuffer it may write to. A machine whose loader passed no framebuffer has none at all. In both cases the serial console is the only record; see [logging](logging.md).

## The two refusals before init

`microkernel_main` checks two things before it creates the first process. Nothing has been started yet, so there is nothing to undo.

If the person chose to install from the boot menu and the image was built without first-boot setup or without the installer capsule, `HAS_INSTALLER` is false (`src/kernel_core/init/entry/install_refusal.rs:24-25`). `refuse_install_without_installer` then shows `Install NONOS: this image has no installer`, says that nothing was written to any disk and that the person should restart and choose another entry, and halts (`src/kernel_core/init/entry/install_refusal.rs:34-47`).

If the kernel's own check of the bootloader refused it, or the boot carried no boot-root record or loader trailer to check, `refuse_unchecked_loader` shows `The bootloader failed the kernel's check` or `The bootloader could not be checked`, says that no program was started, and halts (`src/kernel_core/init/entry/loader_refusal.rs:27-48`). How that check works is on [boot chain and signatures](../security/boot-chain-and-signatures.md).

A refused handoff stops even earlier, before the kernel trusts any framebuffer. That case is on [boot handoff](boot-handoff.md).

## Kernel panic

A Rust panic in the kernel runs `panic` (`src/boot/panic/handler.rs:41-64`):

1. It prints `!!! KERNEL PANIC !!!` and the panic message, with its source location, on the serial console.
2. It drains the debug ring to the console, on kernels built with that feature.
3. It writes `KERNEL PANIC - See serial for details` to VGA text memory, which, as the handler's comment notes, a display in UEFI graphics mode does not show.
4. `show` paints a red band across the top of the framebuffer with `KERNEL PANIC`, the source file and line, and `details on the serial console` (`src/sys/boot_log/panic_screen.rs:26-48`).
5. `send_panic_ipi` stops every other CPU with an NMI, which arrives even at a CPU that spins with interrupts masked (`src/smp/panic_ipi.rs:26-31`). In `on_nmi` each one marks itself halted and stops (`src/smp/nmi/handle.rs:32-48`).
6. It halts.

## CPU exceptions

A page fault, a general protection fault and an invalid opcode first print a `[TRAP xx]` line through `dump_trap`, with the privilege level, the instruction and stack pointers, the code and stack segments, flags, CR3, the address-space id, the pid, the error code and, for a page fault, CR2 (`src/arch/x86_64/diag/dump_trap.rs:23-67`). `xx` is the exception's short name: `PF`, `GP` or `UD`. Other exceptions print no `[TRAP xx]` line.

The page-fault `handle` first tries to resolve the fault, such as a page mapped on demand or a copy-on-write page (`src/interrupts/handlers/exceptions/page_fault.rs:31-53`). If it cannot:

- In user mode, `terminate_user_process` ends the capsule with exit status -11 (`src/interrupts/handlers/exceptions/page_fault.rs:71-77`). The rest of the system keeps running. Other exceptions in user mode end the capsule the same way with their own status, such as -4 for an invalid opcode, passed to `exit_and_yield` (`src/interrupts/handlers/exceptions/opcode.rs:79`).
- In kernel mode, `kernel_panic` prints `[PANIC PF] fatal kernel fault, no recovery path rip=` with the instruction address and `-- CPU halting, boot terminated`, through `emit_fatal_notice` (`src/arch/x86_64/diag/fatal_notice.rs:19-25`), and halts that CPU (`src/interrupts/handlers/exceptions/page_fault.rs:79-95`).

A kernel general protection fault prints its own `[PANIC GP]` line with the selector from the error code, through a local `emit_fatal_notice` (`src/interrupts/handlers/exceptions/gpf.rs:75-86`). A double fault uses `emit_fatal_notice_nolock`, which writes without taking the console lock, since the interrupted code may hold it (`src/interrupts/handlers/exceptions/double_fault.rs:27`).

This path writes to the serial console only. It does not paint the panel and does not signal the other CPUs, so on a machine without a serial port nothing new appears on the panel and the CPU that faulted stops.

## Kernel heap exhausted

When the kernel heap cannot satisfy an allocation, `alloc_error_handler` calls `handle_oom` (`src/lib.rs:52-56`). `handle_oom` prints `[OOM] ALLOCATION FAILED` with the requested size and alignment, dumps the memory-map and surface accounting, prints `[OOM] System halted`, writes `OOM: Memory allocation failed - system halted` to VGA text memory, and halts that CPU (`src/entry/oom.rs:46-63`). The module forbids allocation on this path, which is why `handle_oom` prints the size through the console's own `print_dec` and not the formatter (`src/entry/oom.rs:46-53`). The heap is the 64 MiB described on [memory and paging](memory-and-paging.md).
