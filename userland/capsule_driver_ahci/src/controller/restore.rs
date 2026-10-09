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

//! CAP and PI across the HBA reset. Some of their bits are write-once: the
//! platform firmware writes them at boot (CAP.SSS, CAP.SMPS, CAP.SXS, the
//! ports PI names), and on many Intel PCHs GHC.HR puts them back to their
//! hardware defaults, so after the reset PI can read 0 and CAP can lose
//! staggered spin-up. Linux saves both before the reset and writes them back
//! after (ahci_save_initial_config, ahci_restore_initial_config); so does
//! this. Pure, so the decision is held on the host.

/// What to write back after the reset, given the words read before it
/// (`saved_*`) and after it (`now_*`). A word is written only when it
/// changed, and PI only when the firmware had set it: a zero PI saved is
/// nothing to restore.
pub const fn restore_writes(
    saved_cap: u32,
    saved_pi: u32,
    now_cap: u32,
    now_pi: u32,
) -> (Option<u32>, Option<u32>) {
    let cap = if now_cap != saved_cap { Some(saved_cap) } else { None };
    let pi = if saved_pi != 0 && now_pi != saved_pi { Some(saved_pi) } else { None };
    (cap, pi)
}
