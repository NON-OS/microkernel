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

/*
 * The lines under the review table: what the mode keeps, what may run,
 * the Wi-Fi network, and whether the name, Qwen model and apps are kept. With no
 * store this boot the answers still reach the vfs, and the installer
 * carries them from there to the disk it writes.
 */

use crate::state::Context;

use super::mode;

pub fn mode_line(ctx: &Context) -> &'static [u8] {
    match (mode::keeps(ctx), crate::keep::store_ready()) {
        (false, _) if ctx.install_boot => {
            b"Mode: amnesic. No install this boot; the desktop starts."
        }
        (false, _) => b"Mode: amnesic. Nothing is kept; setup runs again next boot.",
        (true, true) => b"Mode: install. Answers kept, then the installer opens.",
        (true, false) => b"Mode: install. No store here; the installer carries them.",
    }
}

pub fn net_line(ctx: &Context) -> &'static [u8] {
    match (ctx.net.joined.is_some(), ctx.net.remember && mode::keeps(ctx)) {
        (false, _) => b"Wi-Fi: none joined.",
        (true, true) => b"Wi-Fi: joined, remembered sealed with the TPM key.",
        (true, false) => b"Wi-Fi: joined for this boot only; nothing is kept.",
    }
}

pub fn local_line(ctx: &Context) -> &'static [u8] {
    /*
     * Named here too, since this commit is what grants or revokes it.
     */
    match (ctx.local_sel, mode::keeps(ctx)) {
        (1, true) => b"Installed software may run",
        (1, false) => b"Installed software may run, this boot",
        _ => b"Only NONOS software runs",
    }
}

/*
 * keep::save keeps these three with the keyboard, time zone and wallpaper, so
 * they are kept exactly when the mode line says the answers are.
 */
pub fn name_line(ctx: &Context) -> &'static [u8] {
    match (mode::keeps(ctx), crate::keep::store_ready()) {
        (false, _) => b"Name, Qwen model and apps: amnesic, for this boot only.",
        (true, true) => b"Name, Qwen model and apps: kept with the other answers.",
        (true, false) => b"Name, Qwen model and apps: kept on the disk the installer writes.",
    }
}
