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
//! Bringing one root port to Enabled before Address Device (xHCI 1.2
//! sections 4.3 and 4.19, USB 2.0 section 7.1.7.3, Linux `hub_port_reset`).
//!
//! A USB 2 port is enabled only by a port reset. A USB 3 port enables itself
//! once its link trains, so one that already shows PED is used as it is; one
//! whose link fell to Inactive or Compliance comes back only through a warm
//! reset, and any other gets a hot reset. Every write is the neutral PORTSC
//! value with the one action bit added: writing back a PED of one would
//! disable the very port being brought up.
mod action;
mod debounce;
mod power;
mod reset_port;
mod wait_reset;
pub use action::{port_action, PortAction};
pub use power::power_all_ports;
pub use reset_port::reset_port;
