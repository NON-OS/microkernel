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
 * the Wi-Fi network, and whether the name, Qwen model and apps are kept.
 * Install keeps no answers on the disk this boot came from: they reach the
 * vfs in memory, and the installer carries them to the disk it writes.
 */

use crate::state::Context;

use super::mode;

pub fn mode_line(ctx: &Context) -> &'static [u8] {
    match (mode::keeps(ctx), ctx.install_boot) {
        (false, true) => b"Mode: amnesic. No install this boot; the desktop starts.",
        (false, false) => b"Mode: amnesic. Nothing is kept; setup runs again next boot.",
        (true, _) => b"Mode: install. Answers go to the installed disk; none stay here.",
    }
}

pub fn net_line(ctx: &Context) -> &'static [u8] {
    match (ctx.net.joined.is_some(), ctx.net.remember && mode::keeps(ctx)) {
        (false, _) => b"Wi-Fi: none joined.",
        (true, true) => b"Wi-Fi: joined, remembered sealed with the TPM key.",
        (true, false) => b"Wi-Fi: joined for this boot only; nothing is kept.",
    }
}

pub fn route_line(ctx: &Context) -> &'static [u8] {
    match crate::render::screens::route::route(ctx.route_sel) {
        nonos_policy_proto::route::ANYONE => {
            b"Network: the Anyone network for the browser and downloads."
        }
        nonos_policy_proto::route::DIRECT => {
            b"Network: direct; sites and mirrors see this address."
        }
        _ => b"Network: the Nym mixnet for the browser and downloads.",
    }
}

pub fn local_line(ctx: &Context) -> &'static [u8] {
    // Named here too, since this commit is what grants or revokes it.
    match (ctx.local_sel, mode::keeps(ctx)) {
        (1, true) => b"Installed software may run",
        (1, false) => b"Installed software may run, this boot",
        _ => b"Only NONOS software runs",
    }
}

/*
 * The answers record holds these with the keyboard, time zone and
 * wallpaper, so they go exactly where the mode line says the answers do.
 */
pub fn name_line(ctx: &Context) -> &'static [u8] {
    if mode::keeps(ctx) {
        b"Names, Qwen model and apps: kept on the disk the installer writes."
    } else {
        b"Names, Qwen model and apps: amnesic, for this boot only."
    }
}
