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

//! Which chip answers: the version code, the upper half of the PLA dword
//! at 0xe610 masked by 0x7cf0. Each code is
//! named by the bring-up Linux v6.12 gives it (r8152.c); the chip names
//! for the refused codes are inferred from those, not from a datasheet.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Model {
    /// The first RTL8153 silicon: 5c00, 5c10, 5c20, 5c30.
    Rtl8153(u16),
    /// RTL8153B: 6000, 6010.
    Rtl8153B(u16),
}

impl Model {
    pub fn code(self) -> u16 {
        match self {
            Model::Rtl8153(c) | Model::Rtl8153B(c) => c,
        }
    }

    pub fn is_b(self) -> bool {
        matches!(self, Model::Rtl8153B(_))
    }
}

const FIELD: u32 = 0x7cf0;

/// The version code in the dword read from 0xe610.
pub fn code_of(dword: u32) -> u16 {
    ((dword >> 16) & FIELD) as u16
}

/// The chip a code names, or why it is not this driver's.
pub fn identify(code: u16) -> Result<Model, &'static str> {
    match code {
        0x5c00 | 0x5c10 | 0x5c20 | 0x5c30 => Ok(Model::Rtl8153(code)),
        0x6000 | 0x6010 => Ok(Model::Rtl8153B(code)),
        0x4c00 | 0x4c10 | 0x4800 => Err("chip is an RTL8152, not this driver's"),
        0x6400 => Err("chip is an RTL8153C, not this driver's"),
        0x7010 | 0x7020 | 0x7030 => Err("chip is an RTL8156, not this driver's"),
        0x7400 | 0x7410 | 0x7420 => Err("chip is an RTL8156B, not this driver's"),
        _ => Err("chip version unknown, not this driver's"),
    }
}
