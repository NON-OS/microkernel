// NONOS std thread-local keys. Each thread owns one page-sized table of
// pointers; the kernel carries the table's address in the thread pointer
// (set with the MSTB syscall, restored by the scheduler on every switch),
// and slot 0 holds the table's own address so userspace can find it with a
// single load through it (pal thread_regs). Keys are process-global indices
// into that table, handed out from an atomic counter.
//
// Invariant: `init_tls` runs before any `get`/`set` on a thread. The PAL
// runtime init covers the main thread and the thread trampoline covers
// spawned threads; a violation loads through a zero thread pointer, which
// the unmapped null page turns into a loud capsule kill rather than
// silent corruption.
//
// Destructors use a global registration list,
// walked by `destroy_tls` when a thread exits.

use crate::alloc::{GlobalAlloc, Layout, System};
use crate::mem::ManuallyDrop;
use crate::ptr;
use crate::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use crate::sync::atomic::{Atomic, AtomicPtr, AtomicUsize};

pub type Key = usize;
pub type Dtor = unsafe extern "C" fn(*mut u8);

/// One 4 KiB page of pointer slots; slot 0 is the table self-pointer.
const TLS_SLOTS: usize = 512;

/// TLS keys start at 1. Index 0 is the self-pointer, and 0 doubles as the
/// sentinel value `LazyKey` uses for "not yet created".
static TLS_KEY_INDEX: Atomic<usize> = AtomicUsize::new(1);

static DTORS: Atomic<*mut Node> = AtomicPtr::new(ptr::null_mut());

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const N_MK_SET_TLS: i64 = tag4(b"MSTB");
const N_MK_DEBUG: i64 = tag4(b"MDBG");

fn set_tls_base(base: u64) -> i64 {
    #[cfg(target_arch = "riscv64")]
    return crate::sys::pal::thread_regs::set_thread_pointer(base);
    // SAFETY: MSTB takes the new thread pointer and answers a status;
    // the kernel validates the address before installing it.
    #[cfg(not(target_arch = "riscv64"))]
    unsafe {
        crate::sys::pal::trap::trap(N_MK_SET_TLS, base, 0, 0, 0, 0, 0)
    }
}

fn debug_line(msg: &[u8]) {
    // SAFETY: MDBG copies the (pointer, length) bytes to the kernel serial sink and
    // never writes back into the buffer.
    unsafe {
        crate::sys::pal::trap::trap(N_MK_DEBUG, msg.as_ptr() as u64, msg.len() as u64, 0, 0, 0, 0);
    }
}

fn table_layout() -> Layout {
    // TLS_SLOTS pointers always form a valid layout; expect-free unwrap
    // via match keeps this const-friendly and panic-free.
    match Layout::from_size_align(TLS_SLOTS * size_of::<*mut u8>(), size_of::<*mut u8>()) {
        Ok(layout) => layout,
        Err(_) => crate::sys::abort_internal(),
    }
}

fn tls_table_ptr() -> *mut *mut u8 {
    crate::sys::pal::thread_regs::tls_table()
}

// The main thread's table is installed lazily by the first get/set: the
// runtime reads a thread-local (the current thread id) before sys::init
// runs, so no eager hook fires early enough. The flag makes that lazy
// path sound: until it is set, exactly one thread exists (main; spawned
// threads run init_tls in their trampoline before any user code), so the
// first access with the flag clear is always main installing its table.
static MAIN_TLS_READY: Atomic<bool> = crate::sync::atomic::AtomicBool::new(false);

#[inline]
fn ensure_tls() {
    if !MAIN_TLS_READY.load(Acquire) {
        init_tls();
        MAIN_TLS_READY.store(true, Release);
    }
}

/// Allocate this thread's TLS table and hand its address to the kernel.
/// Runs once per thread, before any thread-local access on it.
pub(crate) fn init_tls() {
    // SAFETY: table_layout() is a valid non-zero layout.
    let table = unsafe { System.alloc_zeroed(table_layout()) } as *mut *mut u8;
    if table.is_null() {
        debug_line(b"std tls: table allocation failed\n");
        crate::sys::abort_internal();
    }
    // SAFETY: `table` was just allocated with room for TLS_SLOTS pointers.
    unsafe { *table = table as *mut u8 };
    if set_tls_base(table as u64) != 0 {
        debug_line(b"std tls: MSTB rejected the tls base\n");
        crate::sys::abort_internal();
    }
}

#[inline]
pub fn create(dtor: Option<Dtor>) -> Key {
    let key = TLS_KEY_INDEX.fetch_add(1, Relaxed);
    if key >= TLS_SLOTS {
        // The table is a fixed page; running out means the process created
        // hundreds of distinct thread_local statics. Fail loudly.
        debug_line(b"std tls: out of tls key slots\n");
        crate::sys::abort_internal();
    }
    if let Some(f) = dtor {
        register_dtor(key, f);
    }
    key
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    if key == 0 || key >= TLS_SLOTS {
        crate::sys::abort_internal();
    }
    ensure_tls();
    // SAFETY: the key bound was just checked and the table is live for the
    // whole thread (ensure_tls above or the thread trampoline).
    unsafe { *tls_table_ptr().add(key) = value };
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    if key == 0 || key >= TLS_SLOTS {
        crate::sys::abort_internal();
    }
    ensure_tls();
    // SAFETY: as in `set`.
    unsafe { *tls_table_ptr().add(key) }
}

#[inline]
pub unsafe fn destroy(_key: Key) {
    // Keys index a fixed table and are never reused, so the slot is
    // leaked: a bounded cost against a reused key's stale value.
}

struct Node {
    dtor: Dtor,
    key: Key,
    next: *mut Node,
}

fn register_dtor(key: Key, dtor: Dtor) {
    // The System allocator keeps this off any Global allocator that may
    // itself want thread-local state.
    let mut node =
        ManuallyDrop::new(Box::new_in(Node { key, dtor, next: ptr::null_mut() }, System));
    let mut head = DTORS.load(Acquire);
    loop {
        node.next = head;
        match DTORS.compare_exchange(head, &mut **node, Release, Acquire) {
            Ok(_) => return,
            Err(cur) => head = cur,
        }
    }
}

/// Tear down the calling thread's TLS: run destructors, then release the
/// table. Called by the thread trampoline on exit and by runtime cleanup
/// for the main thread.
pub(crate) unsafe fn destroy_tls() {
    let table = tls_table_ptr();
    if table.is_null() {
        return;
    }
    // SAFETY (caller): TLS was initialized on this thread; run_dtors only
    // touches live table slots.
    unsafe { run_dtors() };
    let _ = set_tls_base(0);
    // SAFETY: `table` came from System.alloc_zeroed with the same layout,
    // and no thread-local access happens after this point.
    unsafe { System.dealloc(table as *mut u8, table_layout()) };
}

// inline(never) keeps dealloc calls from being reordered past the TLS
// teardown.
#[inline(never)]
unsafe fn run_dtors() {
    // Five passes: destructors may themselves
    // create fresh thread locals that also need destroying.
    for _ in 0..5 {
        let mut any_run = false;
        let mut cur = DTORS.load(Acquire);
        while !cur.is_null() {
            // SAFETY: nodes are leaked ManuallyDrop allocations, so every
            // pointer on the list stays valid for the process lifetime.
            let (key, dtor) = unsafe { ((*cur).key, (*cur).dtor) };
            // SAFETY: keys on the dtor list came from `create`, so the
            // bounds hold and the table is live on this thread.
            let ptr = unsafe { get(key) };
            if !ptr.is_null() {
                unsafe {
                    set(key, ptr::null_mut());
                    dtor(ptr);
                }
                any_run = true;
            }
            cur = unsafe { (*cur).next };
        }
        if !any_run {
            break;
        }
    }
    crate::rt::thread_cleanup();
}
