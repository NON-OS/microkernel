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

//! One step of the bring-up (plan.rs): a MAC register written, a PHY
//! register written, or a fixed wait.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    /// The first `len` bytes of `data` to MAC register `reg`.
    Mac {
        reg: u16,
        data: [u8; 8],
        len: u8,
    },
    Phy {
        reg: u16,
        value: u16,
    },
    Pause(u64),
}

pub(super) fn mac(reg: u16, bytes: &[u8]) -> Step {
    let mut data = [0u8; 8];
    data[..bytes.len()].copy_from_slice(bytes);
    Step::Mac { reg, data, len: bytes.len() as u8 }
}

pub(super) const fn phy(reg: u16, value: u16) -> Step {
    Step::Phy { reg, value }
}
