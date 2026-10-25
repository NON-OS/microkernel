// NONOS Operating System (AGPL-3.0-or-later)
//! The trampoline, entered on a fresh stack with the two words the std
//! spawner writes (the function at +0, its argument at +8), calls the one
//! with the other, on that stack.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use crate::enter::{enter, leave};
use crate::thread_regs::thread_entry;

static SEEN: AtomicU64 = AtomicU64::new(0);
static SP_AT_ENTRY: AtomicUsize = AtomicUsize::new(0);

extern "C" fn landed(arg: *mut u64) -> ! {
    let marker = 0u8;
    SP_AT_ENTRY.store(&marker as *const u8 as usize, Ordering::SeqCst);
    // SAFETY: the test passes the address of a live u64 it owns.
    SEEN.store(unsafe { *arg }, Ordering::SeqCst);
    leave()
}

/// Run the trampoline as a spawned thread would start, and give back the
/// argument the called function saw, and where its stack was.
fn spawn_like_the_kernel(arg: u64) -> (u64, usize, std::ops::Range<usize>) {
    SEEN.store(0, Ordering::SeqCst);
    let stack: &'static mut [u64] = Vec::leak(vec![0u64; 16 * 1024]);
    let arg: &'static mut u64 = Box::leak(Box::new(arg));
    let base = stack.as_mut_ptr() as usize;
    let top = (base + stack.len() * 8) & !15;
    let entry_sp = top - 16;
    let start = landed as extern "C" fn(*mut u64) -> !;
    // SAFETY: entry_sp .. top lies inside the leaked stack, as in Thread::new.
    unsafe {
        (entry_sp as *mut u64).write(start as usize as u64);
        ((top - 8) as *mut u64).write(arg as *mut u64 as u64);
    }
    let entry = thread_entry as *const () as usize;
    // SAFETY: the stack is live and 16-aligned, and `landed` ends the thread.
    // The thread ends with a bare exit, which std's join would read as a
    // thread lost, so it is left detached and its answer waited for.
    drop(std::thread::spawn(move || unsafe { enter(entry, entry_sp) }));
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while SEEN.load(Ordering::SeqCst) == 0 && std::time::Instant::now() < deadline {
        std::thread::yield_now();
    }
    (SEEN.load(Ordering::SeqCst), SP_AT_ENTRY.load(Ordering::SeqCst), base..top)
}

#[test]
fn the_trampoline_calls_the_function_with_its_argument_on_that_stack() {
    let (seen, sp, stack) = spawn_like_the_kernel(0x0005_eed0_f57d);
    assert_eq!(seen, 0x0005_eed0_f57d);
    assert!(stack.contains(&sp), "the function ran off the given stack: {sp:#x}");
}
