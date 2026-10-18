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

//! When a CPU may turn the trampoline on and when a user root carries it:
//! pure steps over one state word, so the proofs walk every order.
//!
//! A CPU that leaves for EL0 through the trampoline must find it in every
//! root it can run, so the choice is frozen by the first user root: an
//! exposed CPU that comes before it turns the trampoline on for all roots
//! to come; one that comes after a root made without it stays exposed and
//! says so, rather than take an exception through a root that lacks it.

/// The tables are not in: nothing switches, no root carries them.
pub const IDLE: u8 = 0;
/// The tables are in and no user root exists yet.
pub const OPEN: u8 = 1;
/// Some CPU is exposed: every user root carries the trampoline.
pub const ON: u8 = 2;
/// A user root was made first: none ever carries it.
pub const OFF: u8 = 3;

/// The tables are in.
pub const fn built(now: u8) -> u8 {
    if now == IDLE {
        OPEN
    } else {
        now
    }
}

/// An exposed CPU asks to use the trampoline: the next state, and whether
/// it may.
pub const fn exposed(now: u8) -> (u8, bool) {
    match now {
        OPEN | ON => (ON, true),
        _ => (now, false),
    }
}

/// A user root is being made: the next state, and whether it carries the
/// trampoline.
pub const fn root(now: u8) -> (u8, bool) {
    match now {
        ON => (ON, true),
        _ => (OFF, false),
    }
}
