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

//! The walk as driver.usb_hub0 runs it: a pass over every hub's ports, then
//! a look every 10 ms at the ports mid bring-up until none is, the clock
//! moving between looks; then a pass's wait.

use nonos_usb_hub::{scan_hubs, Classes, Hub};

use super::state::Fake;

const HUB_LOOK_MS: i64 = 10;
const PASS_MS: i64 = 250;
/// Looks one walk takes at most: a port's bring-up is a second or less.
const LOOKS: usize = 200;

pub fn walk(h: &mut Fake, hubs: &mut Vec<Hub>, c: &mut impl Classes<Fake>) {
    let mut busy = scan_hubs(h, hubs, c, false);
    for _ in 0..LOOKS {
        if !busy {
            break;
        }
        h.now += HUB_LOOK_MS;
        busy = scan_hubs(h, hubs, c, true);
    }
    h.now += PASS_MS;
}
