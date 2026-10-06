// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What became of the chosen wallpaper, on the serial line: which step kept
//! it off the desktop, and that it is on. The subscriber tries again every
//! few seconds, so each wallpaper's failing step is said once until it is
//! shown; a fetch that stops says how far it came, so the log tells a store
//! read that never came back (byte 0) from a call lost along the way. That
//! it is shown is said every time, since nothing else on the line says the
//! desktop left its built in picture.

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

/// The wallpapers whose failure has been said, one bit per index.
static SAID: AtomicU64 = AtomicU64::new(0);

/// Say "[WALLPAPER] wallpaper <index>: <step> failed", the first time.
pub fn failed(index: u8, step: &str) {
    if !first(index) {
        return;
    }
    let mut line = start(index);
    line.extend_from_slice(b": ");
    line.extend_from_slice(step.as_bytes());
    line.extend_from_slice(b" failed\n");
    say(&line);
}

/// Say "[WALLPAPER] wallpaper <index>: fetching it from the catalog stopped
/// at byte <at> of <size>", the first time.
pub fn stopped(index: u8, at: u32, size: u32) {
    if !first(index) {
        return;
    }
    let mut line = start(index);
    line.extend_from_slice(b": fetching it from the catalog stopped at byte ");
    decimal(&mut line, at);
    line.extend_from_slice(b" of ");
    decimal(&mut line, size);
    line.push(b'\n');
    say(&line);
}

/// Say "[WALLPAPER] wallpaper <index> shown", and say its next failure again.
pub fn shown(index: u8) {
    SAID.fetch_and(!bit(index), Ordering::Relaxed);
    let mut line = start(index);
    line.extend_from_slice(b" shown\n");
    say(&line);
}

fn bit(index: u8) -> u64 {
    1u64.checked_shl(index as u32).unwrap_or(0)
}

/* Whether this is the first failure of `index` said since it was shown. */
fn first(index: u8) -> bool {
    let bit = bit(index);
    bit == 0 || SAID.fetch_or(bit, Ordering::Relaxed) & bit == 0
}

fn start(index: u8) -> Vec<u8> {
    let mut line = Vec::with_capacity(96);
    line.extend_from_slice(b"[WALLPAPER] wallpaper ");
    decimal(&mut line, index as u32);
    line
}

fn decimal(line: &mut Vec<u8>, mut n: u32) {
    let mut digits = [0u8; 10];
    let mut i = digits.len();
    loop {
        i -= 1;
        digits[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    line.extend_from_slice(&digits[i..]);
}

fn say(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
