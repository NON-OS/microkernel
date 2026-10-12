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

#![no_std]

//! The OS side (OPM) of the USB Type-C Connector System Software Interface:
//! the mailbox firmware shares with the platform's connector manager (PPM),
//! the commands it takes, what they answer, and a session that keeps
//! reading each connector's state when the PPM sticks or floods. Pure: the
//! mailbox is reached through the caller's `Ppm`, and nothing here waits
//! except through it. Read-only: no role swap, no alternate mode.
//!
//! Every constant is in `layout.rs`, `command.rs`, `status.rs` and
//! `capability.rs`, written from the UCSI specification as recalled
//! (revision 1.2, with 2.0's larger message); the published document
//! could not be reached when this was written, so each is marked so.

extern crate alloc;

pub mod capability;
pub mod command;
pub mod exchange;
pub mod layout;
mod poll;
pub mod ppm;
mod recover;
mod serve;
pub mod session;
pub mod start;
pub mod status;
pub mod storm;
