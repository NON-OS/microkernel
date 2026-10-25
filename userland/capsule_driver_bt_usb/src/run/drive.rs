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

//! The capsule's life: find a controller, run it until it fails, give its
//! slot back, and look again.

use nonos_usbnet::xhci::disable_slot;

use super::bonds;
use super::controller::controller;
use super::finder::Finder;
use super::say::{reason, say};

/// Between two looks at the ports while no controller is bound.
const SEARCH_MS: u64 = 1_000;

pub fn run() -> ! {
    let (mut env, irk) = bonds::environment();
    let mut finder = Finder::new();
    say(&[b"started, looking for a controller"]);
    loop {
        let Some((bound, port)) = finder.pass(&env.desk) else {
            env.desk.wait(SEARCH_MS);
            continue;
        };
        let (xhci, slot) = (bound.pipes.bus.xhci, bound.pipes.bus.slot);
        let why = controller(bound, &irk, &mut env);
        env.desk.board().controller = false;
        say(&[b"controller given back: ", reason(&why)]);
        disable_slot(xhci, slot);
        finder.lost(port);
    }
}
