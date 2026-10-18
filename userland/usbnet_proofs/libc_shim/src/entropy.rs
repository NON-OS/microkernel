// NONOS Operating System (AGPL-3.0-or-later)
//! CryptoRandom, with a switch. The station address is drawn from it, so a
//! proof turns it off to see the core refuse a device rather than fall back
//! to the address the device carries. The bytes are a fixed sequence per
//! thread: different on every draw, the same on every run.

use std::cell::Cell;

thread_local! {
    static ON: Cell<bool> = const { Cell::new(true) };
    static STATE: Cell<u64> = const { Cell::new(0x9E37_79B9_7F4A_7C15) };
}

/// Whether CryptoRandom answers on this thread.
pub fn set_entropy(on: bool) {
    ON.with(|c| c.set(on));
}

pub fn crypto_random(buf: *mut u8, len: usize) -> i64 {
    if !ON.with(|c| c.get()) {
        return -1;
    }
    if buf.is_null() {
        return -14;
    }
    // SAFETY: the core passes a pointer to a buffer of `len` bytes it owns.
    let out = unsafe { core::slice::from_raw_parts_mut(buf, len) };
    for b in out.iter_mut() {
        // xorshift64: not random, only different on each call.
        let mut x = STATE.with(|c| c.get());
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        STATE.with(|c| c.set(x));
        *b = x as u8;
    }
    len as i64
}
