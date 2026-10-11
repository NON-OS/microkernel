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

/// Why a container was refused, one per step of the order. Stable, so a
/// verifier may log the code; nothing about the proof beyond it is said.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    /// Not an NPRF v1 container, or a section the kind needs is missing.
    Format,
    /// A kind this version cannot check yet.
    Kind,
    Record,
    /// The device section does not decode, or is not bound to this record.
    DeviceBinding,
    /// Roots, depth, listing or day the verifier did not accept.
    DeviceTrust,
    DeviceProof,
    Measurements,
    Policy,
    AppId,
    /// Hc or A in the public words disagrees with the record or the policy.
    Publics,
    Isolation(u32),
}

impl Refusal {
    pub const fn code(self) -> u32 {
        match self {
            Refusal::Format => 1,
            Refusal::Kind => 2,
            Refusal::Record => 3,
            Refusal::DeviceBinding => 4,
            Refusal::DeviceTrust => 5,
            Refusal::DeviceProof => 6,
            Refusal::Measurements => 7,
            Refusal::Policy => 8,
            Refusal::AppId => 9,
            Refusal::Publics => 10,
            Refusal::Isolation(c) => 100 + c,
        }
    }
}
