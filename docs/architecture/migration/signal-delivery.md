# Migration: kernel signal delivery

Finding: NKA-2026-031 (informational, latent). Target design: [07-target-object-model.md](../07-target-object-model.md), section "Faults, exits and the foreign mechanism".

## Current state

`src/process/signal` (2,057 phys lines) holds a POSIX-style signal subsystem: per-process pending and blocked sets, a sigaction table, queued `siginfo`, a frame builder that pushes a `SigFrame` onto the user stack, and `sigreturn`.

Who uses which part, at `551bbd34`:

| Part | Reached from | Effect today |
|---|---|---|
| Fault to signal | Every user fault: `deliver_user_fault` maps the fault kind to a signal number (`src/arch/trap/contract/signal.rs:20-29`) and terminates (`src/arch/trap/contract/policy.rs:23-30`) | The process ends with code 128 + signal |
| `MkKill` | `sys_kill` accepts only `SIGINT`, `SIGTERM` and `SIGKILL` (`src/syscall/microkernel/kill.rs:27`) and terminates the target | The process ends |
| Undeclared system call | `SIGSYS` termination (`src/syscall/contract/dispatch.rs:61`) | The process ends |
| `SIGALRM` | `alarm::tick` runs on every tenth timer tick (`src/interrupts/timer/clock.rs:44-45`) and sends `SIGALRM` to expired alarms (`src/process/alarm.rs:25-31`) | Nothing: `Pcb::set_alarm` (`src/process/core/pcb_ops.rs:46`) has no caller, so no alarm is ever set |
| Delivery on syscall return | `syscall.S` calls `syscall_return_signal_hook` after every system call (`src/arch/x86_64/asm/syscall.S:101-106`), which captures a `Context` and runs `run_on_syscall_return` (`src/arch/x86_64/syscall/manager/signal_return.rs:31-40`, `src/process/signal/delivery/syscall_return.rs:22-51`) | For a pending signal with the default action: terminate, stop or continue (`src/process/signal/delivery/default.rs:43-70`). Nothing raises one except the paths above, which terminate directly |
| Handler frames | `dispatch_one` calls `install_handler_frame` when the action is a handler (`src/process/signal/delivery/dispatch.rs:37-43`), which pushes a frame and rewrites the saved context (`src/process/signal/delivery/install.rs:25-53`) | Unreachable: `SignalState::set_action` (`src/process/signal/state/actions.rs:25-27`) has no caller and no system call installs a handler |
| `sigreturn` | `sigreturn_current` (`src/process/signal/delivery/sigreturn.rs:21-36`) | Unreachable: no system call reaches it |
| procfs status | `src/fs/procfs/pid/status.rs:47`, `src/fs/procfs/pid/stat.rs:56` read the signal state | Dead with procfs |

### Who needs signal frames at all

Two kinds of program could:

- **Native capsules.** None installs a handler; there is no `sigaction` call. Capsules written against `nonos_libc` handle faults by exiting, and Rust capsules use panics, which abort.
- **Linux guests.** They need real POSIX signals with `rt_sigframe`, and they already get them from userland. The personality keeps actions, masks, alternate stacks and queued signals itself, and installs a handler by asking the kernel for the parked guest's registers (`MkForeignContext`, `src/process/foreign/signal_call.rs:32`) and answering with a whole new context (`MkForeignSignal`, `src/process/foreign/signal_call.rs:46-63`). The kernel captures that parked frame in the assembly entry path, where the callee-saved registers are still the user's (`src/arch/x86_64/asm/syscall.S:49-53`), and never builds a frame of its own.

So the kernel's handler-frame code serves nobody.

## The boundary failure

Ring 0 builds a user-visible structure from kernel state. `capture_user_context` takes `rbx` and `r12`-`r15` from live registers (`src/arch/x86_64/syscall/manager/signal_return.rs:44`, `src/arch/x86_64/syscall/manager/signal_return.rs:84-111`) after the hook's prologue may have reused them, and sets `rdx`, `rsi` and `rdi` to zero (`src/arch/x86_64/syscall/manager/signal_return.rs:49-51`), although `syscall.S` saved all of them. If any path installed a handler, kernel values would be written to the user stack and the user's values lost on return. The class is "the kernel composes user-visible state from its own registers and memory"; the cure is to compose none.

## Target design

- The kernel has no signals. It has three events, each a fixed action or a message:
  - **Fault**: delivered to the thread's fault endpoint as a message carrying the fault kind, address and the registers saved at entry; with no fault endpoint, the process terminates with code `128 + n`, where `n` is today's mapping (`src/arch/trap/contract/signal.rs:20-29`), so exit codes stay the same.
  - **Kill**: `process_kill(p, code)` on a Process capability with `Kill`; the code is the exit code.
  - **Timer**: `timer_arm` signals a notification bit; a capsule that wants alarm semantics waits on it.
- Registers are read and written only from the frame the entry assembly saved, through `thread_read_regs` and `thread_write_regs` on a suspended or parked thread. The Linux personality's `MkForeignContext` and `MkForeignSignal` become exactly those two calls.
- POSIX signal semantics for native programs, if ever wanted, live in `nonos_libc` over a fault endpoint and a notification, as the personality already does for guests.

## Interface specification

```rust
/// Message on a thread's fault endpoint. 32 words.
#[repr(C)]
pub struct FaultMessage {
    pub kind: u32,          // 1 page, 2 protection, 3 invalid opcode, 4 alignment, 5 arithmetic, 6 other
    pub _pad: u32,
    pub address: u64,       // CR2 for page faults, 0 otherwise
    pub error_code: u64,    // the CPU's error code where it pushes one
    pub regs: SavedRegs,    // as saved at entry, the same layout thread_read_regs returns
}

/// Layout shared by the fault message, thread_read_regs and thread_write_regs:
/// the 17 words of today's syscall frame (src/arch/x86_64/asm/syscall_frame.inc),
/// plus rip, rsp, rflags and the fs base.
#[repr(C)]
pub struct SavedRegs { pub words: [u64; 21] }
```

The reply to a fault message is either `Resume { regs: SavedRegs }` (the kernel sanitises RFLAGS and checks RIP is canonical, as `MkForeignSignal` does through `from_words` today) or `Terminate { code: i32 }`.

During the migration no new interface is needed: steps 1 to 3 below only delete code.

## Migration steps

1. **Delete the unreachable handler path.** Remove `install_handler_frame`, `pick_trampoline`, `SigFrame` and `push_to_user_stack`, `sigreturn_current`, `SignalState::set_action`, the sigaction table and `siginfo` ABI types. `dispatch_one` keeps only the default actions.
   *Invariant*: every observable behaviour is unchanged, since none of the removed code is reachable. *Test*: the build with every profile; the existing kill and fault tests; the NKA-2026-031 disassembly check now finds no capture of live callee-saved registers because `read_callee_saved` is gone.
2. **Remove the syscall-return hook.** With no handler path, a pending signal can only be one whose default action ends or stops the process, and every path that raises one (fault, kill, `SIGSYS`) already terminates directly. Delete `syscall_return_signal_hook` and its call (`src/arch/x86_64/asm/syscall.S:101-106`), `run_on_syscall_return`, `send_signal`, `send_signal_info`, `send_signal_to_group` and `alarm::tick` with the unset alarm field.
   *Invariant*: the system-call return path restores exactly the registers `syscall.S` saved. *Test*: a ring-3 test that sets `rbx`, `r12`-`r15`, `rdi`, `rsi` and `rdx`, makes each system call in the table and checks them afterwards; it passes today and must keep passing.
3. **Reduce `process/signal` to exit codes.** What remains is the fault-kind mapping and the signal-number constants used for exit codes (`128 + n`), about 150 lines.
   *Invariant*: exit codes for faults and kills are unchanged. *Test*: the existing exit-code tests in `docs/kernel/processes-and-spawn-exit.md`'s cases.
4. **Fault endpoints.** With the object model (phase 3 of [13-migration-plan.md](../13-migration-plan.md)), a thread created with a fault endpoint receives `FaultMessage` in place of termination; the personality's guests get the personality as their fault endpoint, replacing the foreign mechanism's fault path.
   *Invariant*: a thread without a fault endpoint terminates exactly as before. *Test*: a capsule that dereferences null exits with 139 (`128 + SIGSEGV`) as today; a test supervisor that receives the fault and resumes the thread at a recovery address.
5. **Rename the foreign context calls.** `MkForeignContext` and `MkForeignSignal` become `thread_read_regs` and `thread_write_regs` on the guest's Thread capability. The `DELIVER` and `SIGRETURN` kinds (`src/process/foreign/signal_call.rs:29-30`) both become a register write followed by a resume.
   *Invariant*: Linux guest signal tests in `userland/capsule_linux_proofs` pass unchanged.

## Does the in-place fix stay necessary?

The audit offers two remedies: read the callee-saved registers from the saved frame, or delete the unreachable path. Step 1 is the second, and after it the first has nothing to fix. If the in-place fix takes the first, step 1 deletes its result; the fix is not wasted, since it removes the hazard until step 1 ships.

## Risks

- **A hidden reader of the signal state.** procfs reads it, and procfs is dead. A grep for `.signals` outside `src/process/signal` at `551bbd34` finds only the two procfs files.
- **Stopped processes.** The `Stop` and `Continue` defaults (`src/process/signal/delivery/default.rs:51-67`) are reachable only through a pending stop signal, which nothing raises. If a job-control feature is planned for native capsules, it belongs to procmgr over `thread_suspend` and `thread_resume`.
- **Exit-code compatibility.** Shells and the terminal read `128 + n`; step 3 keeps the numbers.
