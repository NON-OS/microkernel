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

//! The real recovery and the request it follows, against the fake.

#[cfg(test)]
mod abort_tests;
#[cfg(test)]
mod after;
#[cfg(test)]
mod behind_stall_tests;
#[cfg(test)]
mod budget_tests;
#[cfg(test)]
mod fresh_tests;
#[cfg(test)]
mod hmb_tests;
#[cfg(test)]
mod inbox;
#[cfg(test)]
mod kernel_tests;
#[cfg(test)]
mod next_tests;
#[path = "../../../capsule_driver_nvme/src/recover/plan.rs"]
mod plan;
#[cfg(test)]
mod plan_tests;
#[cfg(test)]
mod rebound_tests;
#[path = "../../../capsule_driver_nvme/src/recover/rebuild.rs"]
mod rebuild;
#[path = "../../../capsule_driver_nvme/src/recover/request.rs"]
mod request;
#[cfg(test)]
mod request_tests;
#[cfg(test)]
mod reset_tests;
#[cfg(test)]
mod script;
#[cfg(test)]
mod sender_tests;
#[cfg(test)]
mod served_tests;
#[cfg(test)]
mod server;
#[cfg(test)]
mod stall_tests;
#[path = "../../../capsule_driver_nvme/src/recover/stalled.rs"]
mod stalled;
#[path = "../../../capsule_driver_nvme/src/recover/state.rs"]
mod state;
#[cfg(test)]
mod state_tests;
#[cfg(test)]
mod stream_tests;
#[cfg(test)]
mod window_tests;

pub use plan::{recover, Stuck, ABORT_GRACE_MS};
pub use request::io_request;
pub use stalled::{Serving, Stalled};
pub use state::Recovered;
