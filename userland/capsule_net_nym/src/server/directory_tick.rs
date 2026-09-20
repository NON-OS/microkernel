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

use core::sync::atomic::Ordering;

use super::directory_clock::{now_ms, NEXT_TRY_MS};
use super::directory_outcome::record;
use crate::directory_sync::sync_step;
use crate::setup;
use crate::state::{directory_exit_count, directory_gateway_count};
use crate::trace;


/// Fetch the node list while nothing is asking to be served.
pub fn directory_tick() {
    // Not done until the directory carries both a gateway and an exit.
    if directory_gateway_count() > 0 && directory_exit_count() > 0 {
        return;
    }
    if now_ms() < NEXT_TRY_MS.load(Ordering::Relaxed) {
        return;
    }
    let tcp_port = setup::tcp_port();
    if tcp_port == 0 {
        return;
    }

    trace::say(b"directory: fetching");
    record(sync_step(tcp_port));
}

