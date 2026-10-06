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

//! The system keys, which the input router hands only to the shell whatever
//! has focus: the volume keys set the master volume and say what plays, the
//! power key says what it can do.

use crate::state::system_key::{system_key, SystemKey, POWER_OFF_UNAVAILABLE};
use crate::state::toast::TOAST_TEXT_MAX;
use crate::state::volume::{is_volume_notice, notice};
use crate::state::{Context, NotifyLevel};

/// Act on `code` if it is a system key, and say whether it was.
pub fn key(ctx: &mut Context, code: u32) -> bool {
    let Some(key) = system_key(code) else { return false };
    let now = crate::server::toast_clock::now();
    match key {
        SystemKey::Volume(key) => {
            let heard = crate::sound::volume_key(key);
            let mut line = [0u8; TOAST_TEXT_MAX];
            let n = notice(heard, &mut line);
            // Info: the notice is the answer to the key, and an alert tone
            // over a volume change would be heard at the level just left.
            ctx.toasts.replace(&line[..n], NotifyLevel::Info, now, is_volume_notice);
        }
        SystemKey::Power => ctx.toasts.push(POWER_OFF_UNAVAILABLE, NotifyLevel::Info, now),
    }
    true
}
