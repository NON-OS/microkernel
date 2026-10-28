// NONOS Operating System (AGPL-3.0-or-later)

//! The bring-up status on the boot console, the interface the shared files
//! call (`line`, `number`, `hex16`, `debug`, `debug_number`). Unlike the
//! RTL8821CE's, nothing is written until a line is whole: the pieces are
//! held in `HELD` and the finished line goes out in one `mk_debug`, under
//! this driver's tag (`line::Line`). No line carries an address, a network
//! name or a serial: the efuse address is never read out of the map.

pub mod digits;
pub mod line;

use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

use line::{Line, LINE_MAX};

/// Whether the per-request debug trace is emitted (as the RTL8821CE's: off).
const DEBUG: bool = false;

static HELD: [AtomicU8; LINE_MAX] = [const { AtomicU8::new(0) }; LINE_MAX];
static HELD_LEN: AtomicUsize = AtomicUsize::new(0);

/// Add `msg` to the line; each newline in it writes the line so far, whole.
pub fn line(msg: &[u8]) {
    let mut buf = [0u8; LINE_MAX];
    for (b, held) in buf.iter_mut().zip(HELD.iter()) {
        *b = held.load(Ordering::Relaxed);
    }
    let mut l = Line::resume(buf, HELD_LEN.load(Ordering::Relaxed));
    l.push(msg, |whole| {
        let _ = nonos_libc::mk_debug(whole.as_ptr(), whole.len());
    });
    let (buf, len) = l.held();
    for (b, held) in buf.iter().zip(HELD.iter()) {
        held.store(*b, Ordering::Relaxed);
    }
    HELD_LEN.store(len, Ordering::Relaxed);
}

/// A verbose debug piece, kept only when `DEBUG` is on.
pub fn debug(msg: &[u8]) {
    if DEBUG {
        line(msg);
    }
}

/// A verbose debug number, kept only when `DEBUG` is on.
pub fn debug_number(n: u32) {
    if DEBUG {
        number(n);
    }
}

/// `n` in decimal, as a piece of the line.
pub fn number(n: u32) {
    let mut out = [0u8; 10];
    line(digits::decimal(n, &mut out));
}

/// `v` as four hex digits, as a piece of the line.
pub fn hex16(v: u16) {
    line(&digits::hex16(v));
}
