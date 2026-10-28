// NONOS Operating System (AGPL-3.0-or-later)

//! The kernel's randomness (a seeded generator, or none at all when a
//! proof says so), net.core's owner as the registry would name it, and the
//! IPC calls the serve loop makes, which the proofs never reach.

use std::cell::Cell;

thread_local! {
    static SEED: Cell<u64> = const { Cell::new(0x9E37_79B9_7F4A_7C15) };
    static NO_RANDOM: Cell<bool> = const { Cell::new(false) };
    static NET_CORE: Cell<u32> = const { Cell::new(0) };
}

/// Whether the kernel has randomness to give from now on.
pub fn set_random(on: bool) {
    NO_RANDOM.with(|c| c.set(!on));
}

pub fn crypto_random(buf: *mut u8, len: usize) -> i64 {
    if NO_RANDOM.with(Cell::get) {
        return -1;
    }
    for i in 0..len {
        let x = SEED.with(|s| {
            let mut x = s.get();
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            s.set(x);
            x
        });
        // SAFETY: the driver passes a buffer of `len` writable bytes.
        unsafe { *buf.add(i) = x as u8 };
    }
    len as i64
}

pub fn set_net_core(pid: u32) {
    NET_CORE.with(|p| p.set(pid));
}

pub fn mk_service_lookup(_name: *const u8, _len: usize, _port: *mut u32, pid: *mut u32) -> i64 {
    let found = NET_CORE.with(Cell::get);
    if found == 0 {
        return -1;
    }
    // SAFETY: the driver passes its own `u32` for the pid.
    unsafe { *pid = found };
    0
}

pub fn mk_ipc_recv_from(_e: u64, _b: *mut u8, _l: usize, _t: u64, _p: *mut u32) -> i64 {
    -1
}

pub fn mk_ipc_reply(_pid: u32, _buf: *const u8, _len: usize) -> i64 {
    -1
}
