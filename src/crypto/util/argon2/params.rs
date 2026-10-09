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

//! Argon2 cost parameters and the bounds the kernel holds them to.

/// Memory in KiB, passes, and lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Params {
    pub m_kib: u32,
    pub t: u32,
    pub p: u32,
}

/// RFC 9106 section 4, the second recommended option: 64 MiB, three passes,
/// four lanes. The lanes run one after another here, so they cost the
/// kernel nothing over one lane but are kept for the RFC's parameters.
pub const RECOMMENDED: Params = Params { m_kib: 64 * 1024, t: 3, p: 4 };

/// The most memory the kernel lends one derivation, whatever a disk asks for.
pub const MAX_M_KIB: u32 = 64 * 1024;
pub const MAX_T: u32 = 16;
pub const MAX_P: u32 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Argon2Error {
    /// A parameter is outside the RFC's range or the kernel's bounds.
    BadParams,
    /// The kernel heap could not lend the memory the parameters name.
    NoMemory,
}

impl Params {
    /// The block count m' of section 3.2: m rounded down to a multiple of 4p.
    pub(super) fn blocks(&self) -> Result<usize, Argon2Error> {
        let ok = (1..=MAX_P).contains(&self.p)
            && (1..=MAX_T).contains(&self.t)
            && self.m_kib >= 8 * self.p
            && self.m_kib <= MAX_M_KIB;
        if !ok {
            return Err(Argon2Error::BadParams);
        }
        Ok((self.m_kib / (4 * self.p) * 4 * self.p) as usize)
    }
}
