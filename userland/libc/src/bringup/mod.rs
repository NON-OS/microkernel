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

//! Driver bring-up: attempts with backoff for a bounded time, then a clean
//! give-up, so no driver can spin a core on a device that will not come up.

pub mod deadline;
pub mod policy;
pub mod recv_turn;
mod run;
mod say;
pub mod schedule;
mod start;

pub use deadline::{retry_after, BRINGUP_BUDGET_MS};
pub use policy::{decide, EXIT_ABSENT, EXIT_EMPTY, EXIT_GAVE_UP};
pub use recv_turn::{recv_turn, RecvTurn, RECV_PARK_MS};
pub use run::{bring_up, recv_ready};
pub use say::say_absent;
pub use schedule::{
    delay_after, next, total_sleep_ms, Next, BRINGUP_ATTEMPTS, BRINGUP_FIRST_DELAY_MS,
    BRINGUP_MAX_DELAY_MS,
};
pub use start::start_driver;
