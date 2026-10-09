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
 * Which capsules init restarts when they end. These are the services the
 * rest of the system reaches by name: when one ended, nothing started it
 * again, so a single crash in net.sockets left the machine without a
 * network, in vfs without files, and in the compositor without a screen,
 * until a reboot. Drivers are not here: a driver exits on purpose when its
 * device is absent and retries its own bring-up. Apps, setup, the installer
 * and the Linux personality are not here either: each ends on purpose.
 */

/// The lifecycle names, as the spawn plan registers them.
pub(crate) const WATCHED: &[&str] = &[
    "ramfs",
    "vfs",
    "keyring",
    "entropy",
    "crypto",
    "policy",
    "market",
    "attest",
    "net_l2",
    "net_ip",
    "net_udp",
    "net_tcp",
    "net_dns",
    "net_dhcp",
    "net_ntp",
    "net_core",
    "net_sockets",
    "net_nym",
    "net_anon",
    "socks5",
    "audio_server",
    "clipboard",
    "input_router",
    "compositor",
    "wm",
    "desktop_shell",
    "wallpaper",
    "wallpaper_catalog",
    "image_codec",
    "shield",
];

pub(crate) fn is_watched(name: &str) -> bool {
    WATCHED.contains(&name)
}

/// How long a service must have been seen ended before it is restarted. Its
/// names are released when it becomes a zombie, so this is no longer what lets
/// a restart register them; it keeps a service that ends at once from being
/// started again in a tight loop.
pub(crate) const SETTLE_MS: u64 = 1_000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Next {
    /// Running, never started, or not due: nothing to do.
    Leave,
    /// It ended and the restart policy allows another start now.
    Restart,
    /// It ended and has used every restart the policy allows: say so once.
    GiveUp,
}

/// What to do about a watched capsule: `alive` and `started` from its
/// lifecycle state, `due` from its restart policy (CapsuleState::
/// should_respawn), `spent` when it has used every restart, and `dead_for`
/// how long it has been seen ended.
pub(crate) fn next(alive: bool, started: bool, due: bool, spent: bool, dead_for: u64) -> Next {
    if alive || !started {
        return Next::Leave;
    }
    if spent {
        return Next::GiveUp;
    }
    if due && dead_for >= SETTLE_MS {
        Next::Restart
    } else {
        Next::Leave
    }
}
