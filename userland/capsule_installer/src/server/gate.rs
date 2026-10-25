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
//! Who may ask what of the installer (S-13b). Any IPC holder reaches this
//! endpoint, and vfs takes /capsules writes from this installer alone, so
//! the package ops are gated on their sender here, by what the kernel says
//! that sender holds (`mk_caps_of`), never by its name:
//! - PKG_QUERY, PKG_COMMIT and PKG_REMOVE: a launcher (SpawnWindow, Admin
//!   standing in), as the shell's consent flow and the Terminal's `nox pkg`
//!   are; a query makes this installer read a file the sender names.
//! - INSTALL, a paid install's charge: AppInstall or a launcher.
//!
//! The rest need nothing more: a load is bounded by the kernel (S-13), and
//! health and the installed list say nothing a launcher could not.

use nonos_libc::{mk_caps_of, CAPS_OF_APP_INSTALL, CAPS_OF_SPAWN_WINDOW};

use crate::protocol::{OP_INSTALL, OP_PKG_COMMIT, OP_PKG_QUERY, OP_PKG_REMOVE};

/// A message's sender, as the kernel stamped it: its pid and when the
/// message was queued, so a pid reused since is never asked about.
#[derive(Clone, Copy)]
pub struct Sender {
    pub pid: u32,
    pub sent_ms: u64,
}

/// The rights an op needs of its sender, any one of them; None for none.
pub fn needs(op: u16) -> Option<&'static [u64]> {
    match op {
        OP_PKG_QUERY | OP_PKG_COMMIT | OP_PKG_REMOVE => Some(&[CAPS_OF_SPAWN_WINDOW]),
        OP_INSTALL => Some(&[CAPS_OF_APP_INSTALL, CAPS_OF_SPAWN_WINDOW]),
        _ => None,
    }
}

/// Whether `op` may be served for `sender`. Any answer but yes refuses.
pub fn admits(op: u16, sender: Sender) -> bool {
    match needs(op) {
        None => true,
        Some(any) => any.iter().any(|&bit| mk_caps_of(sender.pid, bit, sender.sent_ms) == 1),
    }
}
