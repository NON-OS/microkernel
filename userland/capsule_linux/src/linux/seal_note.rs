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

use super::say::say;

/*
 * Slot 0 of the kernel's seal: the app the family runs, and in a box the
 * box with it (seal_slot.rs), noted once, after MkForeignSpawn and before
 * the first MkForeignStart (plan/10-seal.md), or the family is dropped at
 * its end and no Linux run is kept (I-C6).
 */
pub(super) fn seal_app(app_id: &[u8; 32]) {
    let Some(note) = super::seal_slot::slot0(app_id, super::boxed::armed_hash().as_ref()) else {
        return;
    };
    if nonos_libc::mk_seal_note(nonos_libc::SEAL_APP_ID, &note) < 0 {
        say(b"[LINUX] the seal refused the app's note; this run is not kept\n");
    }
}
