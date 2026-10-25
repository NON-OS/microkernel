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

//! Why a display driver was not taken: the wire step it failed at, the
//! errno the driver or the wait gave (0 when the answer came but did not
//! fit), and the sentence prime has always given. upgrade_from_gop says each
//! distinct failure once on the display log, so a compositor left on GOP
//! names what keeps it there.

use alloc::format;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::say::say;

pub struct Unprimed {
    pub step: &'static str,
    pub code: i32,
    pub what: &'static str,
}

/// The last failure said, as (driver, step, code) folded to one word.
static SAID: AtomicU64 = AtomicU64::new(0);

/// `[DISPLAY] GOP to <driver>: <step> failed, errno <code>: <what>`, once
/// for each distinct (driver, step, code) in a row.
pub fn say_once(driver: &str, u: &Unprimed) {
    let key = driver.bytes().chain(u.step.bytes()).fold(u64::from(u.code as u32) | 1 << 40, |h, b| {
        h.rotate_left(5) ^ u64::from(b)
    });
    if SAID.swap(key, Ordering::Relaxed) != key {
        say(&format!("GOP to {}: {} failed, errno {}: {}", driver, u.step, u.code, u.what));
    }
}
