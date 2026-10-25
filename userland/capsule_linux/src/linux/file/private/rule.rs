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

//! Which paths are the family's own: the private prefixes, and a home only
//! when its program's policy grants FILES_HOME. Without it /home and /root
//! are the shared tree's, which a guest reads and never writes. Pure, so the
//! host proofs hold it.

/// The prefixes kept under the family's own root (names.rs).
pub const PRIVATE: &[&[u8]] = &[b"/tmp", b"/dev/shm", b"/home", b"/root", b"/run", b"/var/tmp"];

/// The private prefixes that are a home.
const HOMES: &[&[u8]] = &[b"/home", b"/root"];

/// True when `visible` is one of the private prefixes or below one, a home
/// counting only when `home` is granted.
pub fn private_for(visible: &[u8], home: bool) -> bool {
    PRIVATE.iter().any(|p| {
        visible.starts_with(p)
            && matches!(visible.get(p.len()), None | Some(b'/'))
            && (!HOMES.contains(p) || home)
    })
}
