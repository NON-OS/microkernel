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

//! What the meter (capsule_payment) decides, without the capsule around it:
//! which channel a charge lands in, whether the owner's budget allows it, the
//! voucher it produces, and the exact request that has the keyring sign it.
//!
//! A voucher is cumulative: the payee settles only the newest, and
//! the settlement contract pays the difference from what it already paid. So
//! the meter must never sign a lower `cumulative` for a channel than one it
//! signed before, and everything here is built to make that impossible.

#![no_std]

extern crate alloc;

pub mod budget;
pub mod caller;
pub mod channel;
mod error;
pub mod escrow;
mod fees;
pub mod flow;
pub mod job;
pub mod mailbox;
pub mod merkle;
pub mod open;
pub mod orders;
pub mod pay;
pub mod persist;
pub mod root;
pub mod settle;
pub mod state;
pub mod timeout;
pub mod typed;
pub mod usage;
pub mod voucher;
pub mod wire;
pub mod words;

pub use error::MeterError;
