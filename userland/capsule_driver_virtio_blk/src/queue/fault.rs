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

//! Why a request did not complete, each by name. A fault that leaves the
//! request with the device stalls the queue, and the device is reset
//! before it is asked again; the reset's log line says which.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// A request is still the device's: nothing is posted over it.
    Stalled,
    /// The device did not answer by the deadline.
    TimedOut,
    /// The device used an entry it was never given, or more than one.
    Foreign,
    /// The device answered with a length the request does not have: it
    /// wrote less than it was given to fill, or claims more than there is.
    Length,
    /// The kernel refused the interrupt wait or its acknowledgement.
    Wait,
    /// The request is larger than the data buffer.
    Oversize,
}

impl Fault {
    /// The state as the log names it.
    pub fn name(self) -> &'static str {
        match self {
            Fault::Stalled => "a request was still the device's",
            Fault::TimedOut => "a request timed out",
            Fault::Foreign => "the device answered out of turn",
            Fault::Length => "the device answered with a length the request does not have",
            Fault::Wait => "the interrupt wait failed",
            Fault::Oversize => "a request was larger than the buffer",
        }
    }
}
