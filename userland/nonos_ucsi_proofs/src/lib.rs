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

//! Proofs for `nonos_ucsi`, built on the host against the crate's own
//! sources, included by path, and run against a model PPM that can be
//! slow, stick, refuse a command, or notify without end.

extern crate alloc;

#[path = "../../nonos_ucsi/src/capability.rs"]
pub mod capability;
#[path = "../../nonos_ucsi/src/command.rs"]
pub mod command;
#[path = "../../nonos_ucsi/src/contract.rs"]
pub mod contract;
#[path = "../../nonos_ucsi/src/exchange.rs"]
pub mod exchange;
#[path = "../../nonos_ucsi/src/layout.rs"]
pub mod layout;
#[path = "../../nonos_ucsi/src/offered.rs"]
pub mod offered;
#[path = "../../nonos_ucsi/src/pdo.rs"]
pub mod pdo;
#[path = "../../nonos_ucsi/src/poll.rs"]
mod poll;
#[path = "../../nonos_ucsi/src/ppm.rs"]
pub mod ppm;
#[path = "../../nonos_ucsi/src/recover.rs"]
mod recover;
#[path = "../../nonos_ucsi/src/serve.rs"]
mod serve;
#[path = "../../nonos_ucsi/src/session.rs"]
pub mod session;
#[path = "../../nonos_ucsi/src/start.rs"]
pub mod start;
#[path = "../../nonos_ucsi/src/status.rs"]
pub mod status;
#[path = "../../nonos_ucsi/src/storm.rs"]
pub mod storm;

#[cfg(test)]
mod model;
#[cfg(test)]
mod model_pdos;
#[cfg(test)]
mod model_ppm;
#[cfg(test)]
mod model_reply;
#[cfg(test)]
mod tests;
