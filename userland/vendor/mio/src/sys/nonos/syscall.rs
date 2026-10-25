// The raw NONOS syscall entry the backend needs: a tag builder and the
// argument-count variants used to reach the net.sockets service (MSVL to
// resolve its port, MICL for a request/reply) and to cooperatively yield
// between poll rounds (MYLD). NONOS has no timed-sleep syscall, so the selector
// bounds its wait with the monotonic clock and yields the core each round.
// Shared by the selector and the net modules.

pub(super) const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

// No-argument syscall (MYLD reschedules the caller).
pub(super) unsafe fn sys0(num: i64) {
    sys6(num, 0, 0, 0, 0, 0, 0);
}

pub(super) unsafe fn sys5(num: i64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64) -> i64 {
    sys6(num, a1, a2, a3, a4, a5, 0)
}

// SAFETY (shared by every caller): each arm traps to the kernel with the tag
// and the arguments in the registers that architecture's NONOS entry reads,
// the same as the std layer's pal trap: x86_64 syscall (rax, rdi..r9, burning
// rcx and r11), aarch64 svc #0 (x8, x0..x5), riscv64 ecall (a7, a0..a5). The
// answer comes back in rax, x0 or a0. Callers pass valid pointer and length
// pairs for the buffers a given tag reads or writes; nothing keeps a borrow
// past the call.
pub(super) unsafe fn sys6(num: i64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64) -> i64 {
    let r: i64;
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("syscall", inout("rax") num => r, in("rdi") a1, in("rsi") a2,
        in("rdx") a3, in("r10") a4, in("r8") a5, in("r9") a6, out("rcx") _, out("r11") _);
    #[cfg(target_arch = "aarch64")]
    core::arch::asm!("svc #0", inlateout("x0") a1 => r, in("x1") a2, in("x2") a3,
        in("x3") a4, in("x4") a5, in("x5") a6, in("x8") num as u64, options(nostack));
    #[cfg(target_arch = "riscv64")]
    core::arch::asm!("ecall", inlateout("a0") a1 => r, in("a1") a2, in("a2") a3,
        in("a3") a4, in("a4") a5, in("a5") a6, in("a7") num as u64, options(nostack));
    r
}
