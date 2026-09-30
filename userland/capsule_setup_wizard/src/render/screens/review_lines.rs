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
 * the Wi-Fi network, and the answers no mode keeps.
 */

use crate::state::Context;

use super::mode;

pub fn mode_line(ctx: &Context) -> &'static [u8] {
    match (mode::keeps(ctx), crate::keep::store_ready()) {
        (false, _) => b"Mode: amnesic. Nothing is kept; setup runs again next boot.",
        (true, true) => b"Mode: install. Answers kept, then the installer opens.",
        (true, false) => b"Mode: install. No NONOS store this boot: nothing is kept.",
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
 * keep::save keeps the keyboard, time zone and wallpaper and nothing else,
 * so these two go to the policy service for this boot in every mode.
 */
pub const THIS_BOOT: &[u8] = b"Name and Qwen model: for this boot only, in every mode.";
