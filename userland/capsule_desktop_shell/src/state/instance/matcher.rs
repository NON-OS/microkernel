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

//! Whether a service name is an app's or one of its instances'.

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

/// The position in `bases` of the app `name` is an instance of.
pub fn app_index_of<'a>(bases: impl IntoIterator<Item = &'a [u8]>, name: &[u8]) -> Option<usize> {
    bases.into_iter().position(|base| is_instance_of(base, name))
}
