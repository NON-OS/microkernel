# The NONOS ABI: Calling convention

Part of [The NONOS ABI](README.md).

## Calling convention

| Architecture | Trap | Number | Arguments, in order | Return |
|---|---|---|---|---|
| x86_64 | `syscall` | `rax` | `rdi`, `rsi`, `rdx`, `r10`, `r8`, `r9` | `rax` |
| aarch64 | `svc #0` | `x8` | `x0`, `x1`, `x2`, `x3`, `x4`, `x5` | `x0` |
| riscv64 | `ecall` | `a7` | `a0`, `a1`, `a2`, `a3`, `a4`, `a5` | `a0` |

Every call takes at most six arguments, each a 64-bit register. Most calls that
take a narrower field read it with `u32_arg` or one of its siblings, and the
dispatch then refuses a value that does not fit with `EINVAL` rather than
cutting it down (`src/syscall/microkernel/narrow.rs:17-35`). Three calls do not:
`sys_mmap` gets its protection and flags words cast to 32 bits
(`src/syscall/microkernel/dispatch/process.rs:65`), `sys_exit` its exit code
(`src/syscall/microkernel/dispatch/process.rs:69`), and `sys_futex_wait` its
expected value (`src/syscall/microkernel/dispatch/process.rs:78`).

On x86_64 the entry stub's own header gives the convention: `rax` holds the
number and `r10` the fourth argument, because the CPU overwrites `rcx`
(`src/arch/x86_64/asm/syscall.S:1-4`). The `syscall` instruction clobbers `rcx`
and `r11`, and the stub restores the caller's `rdi`, `rsi` and `rdx` before it
returns (`src/arch/x86_64/asm/syscall.S:115-123`). `setup_fmask` masks IF, TF,
DF and AC on entry (`src/arch/x86_64/syscall/msr.rs:108-111`). `syscall_handler`
sends a number it does not know to `redirect`, which hands the call of a
[foreign process](../overview/glossary-dma-pool.md#foreign-process) to its supervisor,
and answers `ENOSYS` when there is no supervisor
(`src/arch/x86_64/syscall/manager/entry.rs:38-53`).

Vector 0x80 is not a syscall path in 0.9.2, though `abi/wire.toml` names `int80`
as `gateway_default` (`abi/wire.toml:13`). The gate for `VECTOR_SYSCALL` leads
to `handle`, which only counts the interrupt with `increment_syscalls`
(`src/interrupts/idt/table_irqs.rs:46-48`,
`src/interrupts/handlers/irq/syscall.rs:19-21`).

On aarch64 an `svc` from EL0 arrives with exception class `EC_SVC64`
(`src/arch/aarch64/exceptions/handlers/sync.rs:32-39`); `dispatch` reads the
number from `x8` and writes the result to `x0`
(`src/arch/aarch64/exceptions/handlers/svc.rs:22-31`). On riscv64 a `UserEcall`
reaches `dispatch_ecall`
(`src/arch/riscv64/interrupts/handlers/dispatch.rs:27`), which reads `a7`,
writes `a0` and steps the program counter past the `ecall`
(`src/arch/riscv64/interrupts/handlers/syscall.rs:22-38`). Neither architecture
has a foreign redirect.

Only on x86_64 do both user-side paths trap. `raw_syscall` in `nonos_abi`
returns -38 without trapping on any other architecture
(`userland/nonos_abi/src/raw.rs:41-52`). The libc's `raw` traps with `svc #0` on
aarch64 (`userland/libc/src/syscall/raw/aarch64.rs:33-58`), and its fallback
`raw` returns -38 on riscv64
(`userland/libc/src/syscall/raw/fallback.rs:16-27`). No user code in this tree
issues `ecall`, so on riscv64 the kernel side exists and nothing calls it.

## Return values

A call returns a signed 64-bit value. Zero or more is success, and its meaning
depends on the call: a byte count, a pid, an address, a [claim
epoch](../overview/glossary-claim-epoch.md#claim-epoch), or 0. A negative value is a negated
errno, as `SyscallResult::error` stores it
(`src/syscall/types/result.rs:35-37`). `abi/syscalls.toml` publishes
`errno_range` as -4095 to -1 (`abi/syscalls.toml:17-19`). Some calls answer with
a flag rather than data: `sys_cap_check` returns 1 or 0
(`src/syscall/microkernel/capability/handlers.rs:68-74`), and
`IRQ_WAIT_TIMED_OUT`, 1, is what `MkIrqWait` returns when it slept out its whole
timeout (`src/syscall/microkernel/irq/timeout.rs:19-25`). See
[Errors](errors.md) and [Broker](broker.md).
