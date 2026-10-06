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

use nonos_libc::{mk_exit, mk_idle_ms, say_absent, EXIT_ABSENT};

use super::find_within::find_within;

const DRIVER: &[u8] = b"driver.usb_hid";
/// The kernel registers driver.xhci0 when it spawns the host controller
/// driver, before this one, and drops it when that driver exits. A lookup can
/// still race a spawn that has not finished, so it is retried, asleep in
/// between, for about two seconds; past that there is no controller.
const LOOKUP_ATTEMPTS: u32 = 100;
const LOOKUP_PAUSE_MS: u64 = 20;

/// Find the host controller, or say there is none and leave. This used to
/// retry the lookup with `mk_yield` and no bound, which on a machine without
/// xHCI held a core for as long as the machine ran.
pub fn run() -> ! {
    let found = find_within(LOOKUP_ATTEMPTS, crate::xhci::lookup, || {
        let _ = mk_idle_ms(LOOKUP_PAUSE_MS);
    });
    let Some(port) = found else {
        say_absent(DRIVER);
        mk_exit(EXIT_ABSENT)
    };
    super::poll::run(port)
}
