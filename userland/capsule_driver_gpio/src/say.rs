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

//! The capsule's console lines, one per event worth a reader's eye: a pin
//! not armed, a pin masked for storming, a controller not held.

use alloc::format;
use core::fmt::Debug;
use nonos_libc::mk_debug;

pub fn line(text: &str) {
    let l = format!("driver.gpio0: {}\n", text);
    let _ = mk_debug(l.as_ptr(), l.len());
}

pub fn say(what: &[u8], uid: u16, pin: u16, why: &impl Debug) {
    let what = core::str::from_utf8(what).unwrap_or("?");
    line(&format!("{} on controller {} pin {} ({:?})", what, uid, pin, why));
}
