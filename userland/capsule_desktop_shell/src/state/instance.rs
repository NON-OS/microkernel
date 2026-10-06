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

//! Which app a registered service name belongs to. The dock starts extra
//! windows of an app as numbered instances: the kernel registers the first
//! window under the app's own name ("app.terminal") and each further one
//! under the name its signed spawn table declares, the base name, a dot and a
//! decimal slot ("app.terminal.1" to "app.terminal.3"). Every place the shell
//! asks which app a window or a request came from goes through this one
//! matcher, so an instance counts as its app and nothing else does.

/// Longest service name the shell builds; the kernel caps spawn names at 48.
pub const INSTANCE_NAME_MAX: usize = 48;

/// Slots probed past the base name. The kernel's spawn tables declare at most
/// three per app today (src/userspace/capsule_*/spawn.rs); the spare covers a
/// table that grows by one before the shell is told.
pub const INSTANCE_SLOTS: u32 = 4;

/// True for `base` itself and for `base.N`, N one or more decimal digits.
/// "app.terminal" is not matched by "app.terminal_x", "app.terminalfoo",
/// "app.terminal." or "app.terminal.2b".
pub fn is_instance_of(base: &[u8], name: &[u8]) -> bool {
    if name == base {
        return true;
    }
    let Some(rest) = name.strip_prefix(base) else { return false };
    let Some(slot) = rest.strip_prefix(b".") else { return false };
    !slot.is_empty() && slot.iter().all(u8::is_ascii_digit)
}

/// The name of `base`'s window in `slot`: slot 0 is the base name itself,
/// slot N is "base.N". None when the name would not fit.
pub fn instance_name<'a>(
    base: &[u8],
    slot: u32,
    buf: &'a mut [u8; INSTANCE_NAME_MAX],
) -> Option<&'a [u8]> {
    if base.len() > INSTANCE_NAME_MAX {
        return None;
    }
    buf[..base.len()].copy_from_slice(base);
    if slot == 0 {
        return Some(&buf[..base.len()]);
    }
    let mut digits = [0u8; 10];
    let mut n = 0;
    let mut v = slot;
    while v > 0 {
        digits[n] = b'0' + (v % 10) as u8;
        v /= 10;
        n += 1;
    }
    let end = base.len() + 1 + n;
    if end > INSTANCE_NAME_MAX {
        return None;
    }
    buf[base.len()] = b'.';
    for (i, d) in digits[..n].iter().rev().enumerate() {
        buf[base.len() + 1 + i] = *d;
    }
    Some(&buf[..end])
}

/// The position in `bases` of the app `name` is an instance of.
pub fn app_index_of<'a>(bases: impl IntoIterator<Item = &'a [u8]>, name: &[u8]) -> Option<usize> {
    bases.into_iter().position(|base| is_instance_of(base, name))
}
