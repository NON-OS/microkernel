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

//! Why a frame was refused. Each refusal maps to the one reason code the
//! peer is told; the peer never learns more than that code.

use crate::Reason;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireError {
    /// The bytes end before the field they should hold.
    Short,
    /// Bytes are left over after the last field.
    Trailing,
    /// A length is over the cap for this frame or field.
    Oversize,
    /// A reserved field or a flag is not zero.
    Reserved,
    /// The magic is not `NSH1` (or the body's own magic).
    Magic,
    /// A hello's version or suite is not the one this crate speaks.
    Version,
    /// A body inside a frame (a grant, a certificate, a record) has a
    /// version this crate does not read. The peer is told `PROTOCOL`: only
    /// a hello's version is answered with `VERSION`.
    BodyVersion,
    /// A frame type no version of the protocol defines.
    UnknownType,
    /// A known frame type that may not arrive now, or not from this side.
    Order,
    /// A field holds a value outside its range.
    Value,
    /// An encoder was given an output buffer too small for the frame.
    NoRoom,
}

impl WireError {
    /// The reason code sent in `ABORT` for this refusal.
    pub const fn reason(self) -> Reason {
        match self {
            WireError::Version => Reason::Version,
            WireError::NoRoom => Reason::Limit,
            _ => Reason::Protocol,
        }
    }
}
