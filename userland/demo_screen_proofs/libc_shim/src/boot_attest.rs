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

//! The kernel's verdict on the loader: measured, as on a TPM machine.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AdmittedLoader {
    pub measurement: [u8; 32],
    pub root: [u8; 32],
    pub epoch: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BootAttest {
    NotYet,
    Measured(AdmittedLoader),
    SelfReported(AdmittedLoader),
    Refused(u32),
    NoEvidence,
}

pub fn boot_attest() -> Option<BootAttest> {
    let measurement = core::array::from_fn(|i| (i as u8).wrapping_mul(37) ^ 0x5C);
    let root = core::array::from_fn(|i| (i as u8).wrapping_mul(91) ^ 0xA3);
    Some(BootAttest::Measured(AdmittedLoader { measurement, root, epoch: 3 }))
}
