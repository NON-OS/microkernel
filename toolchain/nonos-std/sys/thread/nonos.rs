// NONOS std threads over the MTSP syscall. The kernel clones the current
// capsule's address space handle into a fresh schedulable task, so a thread
// here is a task sharing memory and capabilities with its spawner. The
// spawn ABI carries only an entry point and a stack pointer, so the new
// stack carries two words, thread_start and the ThreadInit pointer, and the
// pal's trampoline (thread_regs) calls the one with the other. Join polls
// MPAL (pid-alive) with a yield between probes; the kernel reaps a dead task
// from the process table within a timer tick, after which its user stack is
// provably untouched and can be freed. Detached threads leak their stack,
// since nobody learns when the task dies.

use crate::alloc::{GlobalAlloc, Layout, System};
use crate::ffi::CStr;
use crate::io;
use crate::num::NonZero;
use crate::thread::ThreadInit;
use crate::time::Duration;

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const N_MK_YIELD: i64 = tag4(b"MYLD");
const N_MK_TIME_MILLIS: i64 = tag4(b"MTMS");
const N_MK_THREAD_SPAWN: i64 = tag4(b"MTSP");
const N_MK_PID_ALIVE: i64 = tag4(b"MPAL");
const N_MK_EXIT: i64 = tag4(b"MEXT");
const N_MK_GETPID: i64 = tag4(b"MGPD");
const N_MK_PROC_STAT: i64 = tag4(b"MPST");

pub const DEFAULT_MIN_STACK_SIZE: usize = 1 << 20;

fn raw_yield() {
    // SAFETY: MYLD takes no arguments and touches no memory.
    unsafe {
        crate::sys::pal::trap::trap(N_MK_YIELD, 0, 0, 0, 0, 0, 0);
    }
}

fn now_ms() -> u64 {
    let r: i64;
    // SAFETY: MTMS takes no arguments and returns the kernel clock.
    unsafe {
        r = crate::sys::pal::trap::trap(N_MK_TIME_MILLIS, 0, 0, 0, 0, 0, 0);
    }
    if r < 0 {
        0
    } else {
        r as u64
    }
}

fn pid_alive(tid: u32) -> bool {
    let r: i64;
    // SAFETY: MPAL takes a pid and answers 0 or 1.
    unsafe {
        r = crate::sys::pal::trap::trap(N_MK_PID_ALIVE, tid as u64, 0, 0, 0, 0, 0);
    }
    r == 1
}

fn thread_spawn(entry: u64, stack: u64) -> i64 {
    let r: i64;
    // SAFETY: MTSP takes (entry, stack) and answers the new tid or a
    // negative errno.
    unsafe {
        r = crate::sys::pal::trap::trap(N_MK_THREAD_SPAWN, entry, stack, 0, 0, 0, 0);
    }
    r
}

pub struct Thread {
    tid: u32,
    stack: *mut u8,
    stack_layout: Layout,
}

// SAFETY: the raw stack pointer is only dereferenced by the spawned thread;
// the handle just owns the allocation for the post-join free.
unsafe impl Send for Thread {}
// SAFETY: same reasoning; the handle itself never aliases the stack memory.
unsafe impl Sync for Thread {}

extern "C" fn thread_start(data: *mut ThreadInit) -> ! {
    // TLS first: everything below (current-thread setup, dtors) needs it.
    crate::sys::thread_local::key::init_tls();
    run_init(data);
    // SAFETY: init_tls ran on this thread and no thread-local access
    // happens after this call.
    unsafe { crate::sys::thread_local::key::destroy_tls() };
    // SAFETY: MEXT never returns; the kernel reaps this task.
    crate::sys::pal::trap::trap_exit(N_MK_EXIT, 0u64);
}

// Kept out of thread_start so no stack slot holding `init` state outlives
// the TLS teardown, where the compiler could otherwise reorder it.
#[inline(never)]
fn run_init(data: *mut ThreadInit) {
    // SAFETY: recreates the box leaked by `Thread::new`; the kernel spawn
    // succeeded, so this thread is the sole owner.
    let init = unsafe { Box::from_raw(data) };
    let rust_start = init.init();
    rust_start();
}

impl Thread {
    pub unsafe fn new(stack: usize, init: Box<ThreadInit>) -> io::Result<Thread> {
        // Sixteen-byte alignment for the ABI, and room above the entry stack
        // pointer for the two words the trampoline reads.
        let size = stack.max(DEFAULT_MIN_STACK_SIZE);
        let size = size.checked_add(15).map(|s| s & !15).unwrap_or(size);
        let layout = Layout::from_size_align(size, 16)
            .map_err(|_| io::const_error!(io::ErrorKind::InvalidInput, "bad stack size"))?;
        // SAFETY: layout is non-zero and valid.
        let base = unsafe { System.alloc(layout) };
        if base.is_null() {
            return Err(io::const_error!(io::ErrorKind::OutOfMemory, "thread stack allocation"));
        }
        let data = Box::into_raw(init);
        let top = base as u64 + size as u64;
        let entry_sp = top - 16;
        let start = thread_start as extern "C" fn(*mut ThreadInit) -> !;
        // SAFETY: top-16..top lies inside the fresh allocation; the spawned
        // thread reads each word once, the function at +0, its argument at +8.
        unsafe {
            (entry_sp as *mut u64).write(start as usize as u64);
            ((top - 8) as *mut u64).write((data as usize) as u64);
        }
        let entry = crate::sys::pal::thread_regs::thread_entry as *const () as usize as u64;
        let tid = thread_spawn(entry, entry_sp);
        if tid <= 0 {
            // The kernel refused the spawn, so the box was never consumed
            // and this thread still owns both allocations.
            unsafe {
                drop(Box::from_raw(data));
                System.dealloc(base, layout);
            }
            return Err(io::const_error!(io::ErrorKind::Uncategorized, "thread spawn rejected"));
        }
        Ok(Thread { tid: tid as u32, stack: base, stack_layout: layout })
    }

    pub fn join(self) {
        while pid_alive(self.tid) {
            raw_yield();
        }
        // The task is gone from the process table, so nothing can execute
        // on this stack anymore.
        // SAFETY: `stack` came from System.alloc with `stack_layout` and is
        // freed exactly once; `self` is consumed here, skipping Drop's leak.
        unsafe { System.dealloc(self.stack, self.stack_layout) };
        crate::mem::forget(self);
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        // Detached thread: its lifetime is unknowable without a wait
        // primitive, so the stack allocation is intentionally leaked.
    }
}

// MkProcStat's header (src/syscall/microkernel/procstat_header.rs), read
// as little-endian words: the layout version is the high half of word 1
// (byte 12), the cores online the low half of word 16 (byte 128), there
// from version 4. Fields are only ever appended, so a newer kernel still
// answers here.
const PROC_STAT_CPUS_SINCE: u32 = 4;
// The kernel's own MAX_CPUS.
const MAX_CPUS: u32 = 256;

pub fn available_parallelism() -> io::Result<NonZero<usize>> {
    // MPST writes its header and one process entry, 320 bytes today; the
    // buffer leaves room for fields a later kernel appends.
    let mut buf = [0u64; 128];
    let r: i64;
    // SAFETY: MPST writes at most the header plus `max_entries` (1) entries
    // into the buffer named first, which is 1 KiB and writable.
    unsafe {
        r = crate::sys::pal::trap::trap(N_MK_PROC_STAT, buf.as_mut_ptr() as u64, 1u64, 0, 0, 0, 0);
    }
    let version = (buf[1] >> 32) as u32;
    // Any refusal or an answer out of range reads as one core, the honest
    // lower bound, so a caller always gets a count it can build on.
    let cpus = if r >= 0 && version >= PROC_STAT_CPUS_SINCE { buf[16] as u32 } else { 1 };
    let cpus = if (1..=MAX_CPUS).contains(&cpus) { cpus as usize } else { 1 };
    Ok(NonZero::new(cpus).unwrap_or(NonZero::<usize>::MIN))
}

pub fn current_os_id() -> Option<u64> {
    let r: i64;
    // SAFETY: MGPD takes no arguments and returns the calling task id.
    unsafe {
        r = crate::sys::pal::trap::trap(N_MK_GETPID, 0, 0, 0, 0, 0, 0);
    }
    if r < 0 {
        None
    } else {
        Some(r as u64)
    }
}

pub fn yield_now() {
    raw_yield();
}

pub fn set_name(_name: &CStr) {}

pub fn sleep(dur: Duration) {
    let ms = dur.as_millis().min(u64::MAX as u128) as u64;
    let start = now_ms();
    while now_ms().wrapping_sub(start) < ms {
        raw_yield();
    }
}
