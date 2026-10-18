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

//! The PS/2 input driver's bring-up, proved without the controller.
//!
//! The `#[path]` includes pull in the shipping driver source, so these tests
//! run the code that boots: discovery, the setup sequence, and every step of
//! the keyboard and mouse bring-up. The driver reaches the i8042 only through
//! two port I/O calls, so the controller model sits behind those calls and
//! answers in the test's own thread. Every byte the driver sends is recorded
//! in order, and every refusal path is reached by a controller built to
//! refuse.
//!
//! The properties that earn this crate its place came from real machines:
//! firmware that hands the port off disabled, a keyboard acknowledgement
//! that lands late and gets taken for the controller configuration, and an
//! aux port with nothing behind it that echoes forever.

#[path = "../../capsule_driver_ps2_input/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_ps2_input/src/discover.rs"]
pub mod discover;
#[path = "../../capsule_driver_ps2_input/src/init/mod.rs"]
pub mod init;
pub mod mouse;
#[path = "../../capsule_driver_ps2_input/src/setup/mod.rs"]
pub mod setup;
// The scan set 1 decode, the flags it reads the prefix and the break bit
// from, the lock keys, the modifiers and the post to the input ring.
pub mod keymap;
// The drain the serve loop runs: each byte routed by its port, keyboard
// bytes framed by their prefixes, decoded and posted, aux bytes assembled
// into packets.
#[path = "../../capsule_driver_ps2_input/src/poll/mod.rs"]
pub mod poll;
pub use poll::prefix;
// The serve loop's state and its upkeep between two waits.
pub mod server;

// The capsule makes its one `Drainer` with the const `new` and has no use
// for `Default`; exported from this library, the type gets one, as clippy
// asks of a public type with a `new`.
impl Default for poll::Drainer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
