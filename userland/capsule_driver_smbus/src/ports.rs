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

//! What the transfer engine needs of the host: its byte ports and a clock
//! for the bounded waits. The capsule's are the PIO grant and the uptime;
//! the proofs' are a model of the controller.

pub trait Ports {
    fn r8(&mut self, off: u16) -> Result<u8, i32>;
    fn w8(&mut self, off: u16, value: u8) -> Result<(), i32>;
    /// Starts a wait of `ms`; `expired` says when it is over.
    fn arm(&mut self, ms: u64);
    fn expired(&mut self) -> bool;
    /// Sleeps about `ms` between polls, so a wait costs wakeups, not a core.
    fn pause(&mut self, ms: u64);
}
