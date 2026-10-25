// NONOS Operating System (AGPL-3.0-or-later)
//! The trampoline, entered on a fresh stack with the two words the std
//! spawner writes (the function at +0, its argument at +8), calls the one
//! with the other, on that stack, aligned as each calling convention wants
//! at a function's first instruction.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use crate::enter::{enter, leave};
use crate::thread_regs::thread_entry;

static SEEN: AtomicU64 = AtomicU64::new(0);
static SP_AT_ENTRY: AtomicUsize = AtomicUsize::new(0);

/// The function the trampoline calls: its first instruction hands the stack
/// pointer, as it is there, to `landed` as a second argument.
#[unsafe(naked)]
extern "C" fn probe(_arg: *mut u64) -> ! {
    #[cfg(target_arch = "x86_64")]
    core::arch::naked_asm!("mov rsi, rsp", "jmp {l}", l = sym landed);
    #[cfg(target_arch = "aarch64")]
    core::arch::naked_asm!("mov x1, sp", "b {l}", l = sym landed);
    #[cfg(target_arch = "riscv64")]
    core::arch::naked_asm!("mv a1, sp", "j {l}", l = sym landed);
}

extern "C" fn landed(arg: *mut u64, sp: usize) -> ! {
    SP_AT_ENTRY.store(sp, Ordering::SeqCst);
    // SAFETY: the test passes the address of a live u64 it owns.
    SEEN.store(unsafe { *arg }, Ordering::SeqCst);
    leave()
}

/// Run the trampoline as a spawned thread would start, and give back the
/// argument the called function saw, and its stack pointer on entry.
pub(crate) fn spawn_like_the_kernel(arg: u64) -> (u64, usize, std::ops::Range<usize>) {
    let _one = crate::enter::ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    SEEN.store(0, Ordering::SeqCst);
    let stack: &'static mut [u64] = Vec::leak(vec![0u64; 16 * 1024]);
    let arg: &'static mut u64 = Box::leak(Box::new(arg));
    let base = stack.as_mut_ptr() as usize;
    let top = (base + stack.len() * 8) & !15;
    let entry_sp = top - 16;
    let start = probe as extern "C" fn(*mut u64) -> !;
    // SAFETY: entry_sp .. top lies inside the leaked stack, as in Thread::new.
    unsafe {
        (entry_sp as *mut u64).write(start as usize as u64);
        ((top - 8) as *mut u64).write(arg as *mut u64 as u64);
    }
    let entry = thread_entry as *const () as usize;
    // The thread ends with a bare exit, which std's join would read as a
    // thread lost, so it is left detached and its answer waited for.
    drop(std::thread::spawn(move || unsafe { enter(entry, entry_sp) }));
    crate::enter::wait_for(|| SEEN.load(Ordering::SeqCst) != 0);
    (SEEN.load(Ordering::SeqCst), SP_AT_ENTRY.load(Ordering::SeqCst), base..top)
}

#[test]
fn the_trampoline_calls_the_function_with_its_argument_on_that_stack() {
    let (seen, sp, stack) = spawn_like_the_kernel(0x0005_eed0_f57d);
    assert_eq!(seen, 0x0005_eed0_f57d);
    assert!(stack.contains(&sp), "the function ran off the given stack: {sp:#x}");
}

#[test]
fn the_callee_starts_on_a_stack_its_calling_convention_accepts() {
    let (_, sp, _) = spawn_like_the_kernel(1);
    // x86_64: a call has pushed the return address onto a 16-aligned stack.
    #[cfg(target_arch = "x86_64")]
    assert_eq!(sp % 16, 8, "rsp {sp:#x}");
    // aarch64 and riscv64 call without pushing: the stack is 16-aligned.
    #[cfg(not(target_arch = "x86_64"))]
    assert_eq!(sp % 16, 0, "sp {sp:#x}");
}
