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

//! Why a fill gave no bytes. Each ends with the queue stalled, and the
//! device is reset before it is asked again.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// A request is still the device's: it timed out, or the device
    /// answered out of turn. Nothing is posted over it.
    Stalled,
    /// The device did not answer by the deadline.
    TimedOut,
    /// The device used an entry it was never given, or more than one.
    Foreign,
}
