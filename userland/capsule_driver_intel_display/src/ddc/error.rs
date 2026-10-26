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

//! Why a DDC/CI exchange gave nothing.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DdcError {
    /// A control the allowlist does not name.
    NotAllowed,
    /// A value the control does not take.
    Value,
    /// The request was not taken: nothing answers DDC/CI on this bus.
    Absent,
    /// The bus did not carry the reply: a NAK, a timeout, a short read.
    Bus,
    /// The reply's length byte is not a VCP reply's.
    Length,
    /// The reply's checksum does not add up.
    Checksum,
    /// The reply is not a VCP reply to the control asked for.
    Reply,
    /// The monitor answers that it does not offer the control (result code
    /// 1, or the null message), or its capabilities do not name it.
    NotOffered,
    /// No clock to pace the bus by.
    NoClock,
    /// The display has not given its capabilities string, or gave a bad one.
    NoCapabilities,
}
