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

/* Which capsules a boot profile does not start. */

use crate::boot::handoff::BootProfile;

const NETWORK_DRIVERS: [&str; 6] = [
    "driver.e1000_0",
    "driver.rtl8139_0",
    "driver.rtl8169_0",
    "driver.virtio_net0",
    "driver.iwlwifi0",
    "driver.rtl8821ce0",
];
/* Safe Mode: no audio, and no game or demo beside the tools. */
const NOT_SAFE: [&str; 4] = ["driver.hda0", "audio.server", "app.snake", "app.hello"];
/* Fetches models over the network. */
const NETWORK_TOOLS: [&str; 1] = ["tool.model-fetch"];

pub(super) fn refused(profile: BootProfile, name: &str) -> bool {
    let network = name.starts_with("net.")
        || NETWORK_DRIVERS.contains(&name)
        || NETWORK_TOOLS.contains(&name);
    let not_safe = NOT_SAFE.iter().any(|n| is_instance(name, n));
    (network && !profile.network()) || (profile.minimal() && not_safe)
}

/* `name` is the capsule `base` or a numbered instance of it, "app.snake.1". */
fn is_instance(name: &str, base: &str) -> bool {
    name.strip_prefix(base).is_some_and(|rest| rest.is_empty() || rest.starts_with('.'))
}
