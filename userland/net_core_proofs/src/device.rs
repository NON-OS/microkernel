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

//! net.core's side of the receive-batch exchange with a NIC driver: the
//! call, the batch body, and what the probe answer is taken to mean. The
//! `#[path]` includes pull in the real source by the `crate::` paths it
//! uses.

#[path = "../../capsule_net_core/src/device/batch_call.rs"]
pub mod batch_call;
#[path = "../../capsule_net_core/src/device/batch_frames.rs"]
pub mod batch_frames;
#[path = "../../capsule_net_core/src/device/batch_served.rs"]
pub mod batch_served;
#[path = "../../capsule_net_core/src/device/rx_seq.rs"]
mod rx_seq;

#[cfg(test)]
mod probe_tests;
