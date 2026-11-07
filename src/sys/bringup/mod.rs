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

//! Per-device bring-up events on serial, for the project's boot traces
//!. Built only with `nonos-bringup-trace`;
//! without it every hook is an empty inline function. One line per event:
//!
//! `[BRINGUP] t=<ms> dev=<name> ev=<event>[ on=<what>][ why=<reason>]`
//!
//! and a summary line at the end of each stage, counting each device by its
//! last event. The ledger is pure; `kernel_proofs` mounts it.

mod ev;
pub mod ledger;
mod not_spawned;

pub use ev::Ev;
pub use not_spawned::not_spawned;

#[cfg(feature = "nonos-bringup-trace")]
mod live;
#[cfg(feature = "nonos-bringup-trace")]
pub use live::{event, summary};

#[cfg(not(feature = "nonos-bringup-trace"))]
mod off;
#[cfg(not(feature = "nonos-bringup-trace"))]
pub use off::{event, summary};
