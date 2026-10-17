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
//! The meter's parts the store's proofs need, mounted where the meter keeps
//! them (nonos_nox_meter): who may call which op, and FUNDED's one bit
//! with the PAY checks it is made of, the channel, budget and state ones.

pub mod budget;
#[path = "../../../nonos_nox_meter/src/caller/mod.rs"]
pub mod caller;
pub mod channel;
#[path = "../../../nonos_nox_meter/src/error.rs"]
pub mod error;
pub mod pay;
pub mod state;
