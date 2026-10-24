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

//! The frame types of nsh/1. A byte that names none of them is refused as
//! `UnknownType`, which is a different refusal from a known type that may
//! not arrive now (`Order`, judged in `expect`).

use crate::WireError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    ClientHello = 0x01,
    ServerHello = 0x02,
    HostProof = 0x10,
    UserAuth = 0x20,
    AuthOk = 0x21,
    AuthRefused = 0x22,
    Input = 0x30,
    Output = 0x31,
    Window = 0x32,
    Signal = 0x33,
    Escalate = 0x40,
    EscalateResult = 0x41,
    Exit = 0x50,
    Record = 0x51,
    Close = 0x52,
    Abort = 0x7F,
}

impl Kind {
    pub const fn from_byte(b: u8) -> Result<Kind, WireError> {
        Ok(match b {
            0x01 => Kind::ClientHello,
            0x02 => Kind::ServerHello,
            0x10 => Kind::HostProof,
            0x20 => Kind::UserAuth,
            0x21 => Kind::AuthOk,
            0x22 => Kind::AuthRefused,
            0x30 => Kind::Input,
            0x31 => Kind::Output,
            0x32 => Kind::Window,
            0x33 => Kind::Signal,
            0x40 => Kind::Escalate,
            0x41 => Kind::EscalateResult,
            0x50 => Kind::Exit,
            0x51 => Kind::Record,
            0x52 => Kind::Close,
            0x7F => Kind::Abort,
            _ => return Err(WireError::UnknownType),
        })
    }

    pub const fn byte(self) -> u8 {
        self as u8
    }
}
