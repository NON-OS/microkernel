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

//! Driver bring-up: a bounded number of attempts with backoff, then a clean
//! give-up, so no driver can spin a core on a device that will not come up.

pub mod policy;
mod run;

pub use policy::{
    decide, delay_after, next, recv_turn, total_sleep_ms, Next, RecvTurn, BRINGUP_ATTEMPTS,
    BRINGUP_FIRST_DELAY_MS, BRINGUP_MAX_DELAY_MS, EXIT_ABSENT, EXIT_GAVE_UP, RECV_PARK_MS,
};
pub use run::{bring_up, recv_ready, say_absent, start_driver};
