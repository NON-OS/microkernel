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

//! A tone when the desktop has finished coming up.

use nonos_policy_client::{get_bool, lookup};
use nonos_policy_proto::Field;

/// 660 Hz for 180 ms: lower and longer than an alert, so the two are not
const CHIME_HZ: u32 = 660;
const CHIME_MS: u32 = 180;

/*
 * Read once, here, rather than polled. This happens exactly once per boot, at the
 * point the shell has its overlay registered and the desktop is on screen, and a
 * value that arrives later is not a request to chime late.
 *
 * Off when the store cannot be reached, which is also the stored default.
 */
pub fn chime() {
    let Some(port) = lookup() else { return };
    if get_bool(port, Field::StartupChime) != Some(true) {
        return;
    }
    super::play::play(CHIME_HZ, CHIME_MS, super::alert::GAIN);
}
