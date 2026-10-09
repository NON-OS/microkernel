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

//! The runner's retry of the wallpaper scaling send. The send itself is owned
//! by the setup the shell primes (`setup::prime::wallpaper_policy`); the runner
//! only asks again on its clock tick until the wallpaper takes it, so a slow or
//! a missing wallpaper never keeps the desktop from coming up.

use super::backoff::Backoff;
use crate::setup::wallpaper_policy::send;
use crate::state::Context;

/// Between tries: each call may wait out its 250 ms reply budget on a busy
/// wallpaper, so the shell asks less often while it stays busy.
static GAP: Backoff = Backoff::new(250, 8000);

/// From the runner: find the wallpaper and send the scaling, until done.
pub fn retry(ctx: &mut Context) {
    if ctx.wallpaper_policy_sent || !GAP.due() {
        return;
    }
    if ctx.wallpaper_port == 0 {
        ctx.wallpaper_port = crate::setup::try_wallpaper();
    }
    ctx.wallpaper_policy_sent = send(ctx.wallpaper_port);
    if !ctx.wallpaper_policy_sent {
        GAP.missed();
    }
}
