// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel's side of a thread spawn, played on Linux: start running at
//! `entry` with the stack pointer at `sp` and nothing else set, and end a
//! thread with the bare exit call, which runs no unwinding and no cleanup on
//! a stack the C library never made. The kernel clears the thread's id word
//! as it goes, which is what lets a join return.

/// Jump to `entry` with the stack pointer at `sp`; never returns.
///
/// # Safety
/// `sp` is the 16-aligned top of a live stack this thread may run on, and
/// `entry` is code that ends the thread rather than return.
pub(crate) unsafe fn enter(entry: usize, sp: usize) -> ! {
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("mov rsp, {sp}", "jmp {e}", sp = in(reg) sp, e = in(reg) entry,
        options(noreturn));
    #[cfg(target_arch = "aarch64")]
    core::arch::asm!("mov sp, {sp}", "br {e}", sp = in(reg) sp, e = in(reg) entry,
        options(noreturn));
    #[cfg(target_arch = "riscv64")]
    core::arch::asm!("mv sp, {sp}", "jr {e}", sp = in(reg) sp, e = in(reg) entry,
        options(noreturn));
}

/// End this thread alone with Linux's exit call (60 on x86_64, 93 on the
/// other two), skipping every destructor.
pub(crate) fn leave() -> ! {
    // SAFETY: exit takes a status and never returns to this thread.
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!("syscall", in("rax") 60u64, in("rdi") 0u64, options(noreturn));
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!("svc #0", in("x8") 93u64, in("x0") 0u64, options(noreturn));
        #[cfg(target_arch = "riscv64")]
        core::arch::asm!("ecall", in("a7") 93u64, in("a0") 0u64, options(noreturn));
    }
}

/// Wait, up to 30 s, for `done`: a thread that ends with a bare exit cannot
/// be joined, so the tests watch for what it leaves behind.
pub(crate) fn wait_for(done: impl Fn() -> bool) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while !done() && std::time::Instant::now() < deadline {
        std::thread::yield_now();
    }
}

/// The probes keep what they saw in statics, so one spawn at a time.
pub(crate) static ONE_AT_A_TIME: std::sync::Mutex<()> = std::sync::Mutex::new(());
