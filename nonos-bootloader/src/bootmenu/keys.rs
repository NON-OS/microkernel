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

use super::entries::{index_of, ENTRIES};
use super::nav::Nav;
use crate::menu::{MenuAction, SecurityMode};
use crate::security::SecurityPolicy;

/// Move the selection by one key. Enter returns the selected entry's action.
pub(super) fn apply(nav: Nav, sel: &mut usize) -> Option<MenuAction> {
    let n = ENTRIES.len();
    match nav {
        Nav::Enter => return Some(ENTRIES[*sel].action),
        Nav::Up => *sel = (*sel + n - 1) % n,
        Nav::Down => *sel = (*sel + 1) % n,
        Nav::First => *sel = 0,
        Nav::Last => *sel = n - 1,
        Nav::Jump(i) if i < n => *sel = i,
        Nav::Jump(_) | Nav::Stop | Nav::None => {}
    }
    None
}

/*
 * A hands-off boot lands on the compile-time floor, which is always
 * satisfiable: Standard, or Hardened on a hardened build. Hardened stays a
 * deliberate choice elsewhere, since it requires Secure Boot and a TPM.
 */
pub(super) fn default_index() -> usize {
    match SecurityPolicy::from_build() {
        SecurityPolicy::Hardened => index_of(MenuAction::Boot(SecurityMode::Hardened)),
        _ => index_of(MenuAction::Boot(SecurityMode::Standard)),
    }
}
