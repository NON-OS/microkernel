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
