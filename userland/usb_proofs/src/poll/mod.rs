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

//! The poll loop's parts: the interrupt read's length rule, beside the
//! report buffer size it is bounded by; the read of one endpoint when it is
//! due; the step that hands a report to the state of the interface it came
//! from; and how often the loop asks each endpoint, waits on its inbox and
//! looks at the ports. The loop around them makes system calls and stays
//! out.

#[path = "../../../capsule_driver_usb_hid/src/orchestrator/poll/constants.rs"]
mod constants;
// The reads and the report step only the tests make, built only for them.
#[cfg(test)]
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/poll/drain_endpoint.rs"]
mod drain_endpoint;
#[cfg(test)]
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/poll/feed_report.rs"]
mod feed_report;
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/poll/pace.rs"]
pub mod pace;
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/poll/period.rs"]
pub mod period;
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/poll/read_len.rs"]
pub mod read_len;
#[path = "../../../capsule_driver_usb_hid/src/orchestrator/poll/rescan.rs"]
pub mod rescan;

pub use pace::Pace;
pub use read_len::read_len;

#[cfg(test)]
mod buffer_tests;
#[cfg(test)]
mod drain_tests;
#[cfg(test)]
mod feeding;
#[cfg(test)]
mod pace_tests;
#[cfg(test)]
mod per_interface_more_tests;
#[cfg(test)]
mod per_interface_tests;
#[cfg(test)]
mod period_tests;
#[cfg(test)]
mod rescan_tests;
