// NONOS Operating System (AGPL-3.0-or-later)
//! libc's worker trampoline (thread/entry.rs), entered on a fresh stack with
//! `Start` written `START_GAP` bytes above it as the spawner writes it, runs
//! the job with its argument and then ends the thread through `mk_exit`.

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::enter::{enter, wait_for, ONE_AT_A_TIME};
use crate::libc_entry::{trampoline, Start, START_GAP};

static SEEN: AtomicUsize = AtomicUsize::new(0);

fn job(arg: usize) {
    SEEN.store(arg, Ordering::SeqCst);
}

#[test]
fn the_worker_runs_its_job_with_its_argument() {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    SEEN.store(0, Ordering::SeqCst);
    let stack: &'static mut [u64] = Vec::leak(vec![0u64; 16 * 1024]);
    let top = (stack.as_mut_ptr() as usize + stack.len() * 8) & !15;
    let first_sp = top - START_GAP - 16;
    let start = Start { job, arg: 0x7ea_cafe };
    // SAFETY: first_sp + START_GAP .. top lies inside the leaked stack and
    // holds exactly one `Start`, as seat.rs lays it out.
    unsafe { ((first_sp + START_GAP) as *mut Start).write(start) };
    let entry = trampoline as *const () as usize;
    // SAFETY: the stack is live and 16-aligned, and the worker ends itself.
    drop(std::thread::spawn(move || unsafe { enter(entry, first_sp) }));
    wait_for(|| SEEN.load(Ordering::SeqCst) != 0);
    assert_eq!(SEEN.load(Ordering::SeqCst), 0x7ea_cafe);
}
