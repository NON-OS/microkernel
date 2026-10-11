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

/// Why a container was refused, one per check of the order. Stable, so a
/// verifier may log the code; nothing about the proof beyond it is said.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    /// Not an NPRF v1 container with exactly its kind's sections.
    Format,
    /// A kind this version cannot check yet.
    Kind,
    /// Proven on a day the verifier did not ask about.
    Window,
    Record,
    /// Not final, saturated, or sealed by a supervisor that is not pinned.
    Supervisor,
    /// The device section does not decode, or is not bound to this record.
    DeviceBinding,
    /// Roots, registry, day or listing the verifier does not accept.
    DeviceTrust,
    DeviceProof,
    Measurements,
    Policy,
    AppId,
    /// Hc or A in the public words disagrees with the record or the policy.
    Publics,
    Isolation(u32),
    /// A delivery body that is not this record's, this service's, or the
    /// buyer's own request, answer, order and channel.
    Body,
}

impl Refusal {
    pub const fn code(self) -> u32 {
        match self {
            Refusal::Format => 1,
            Refusal::Kind => 2,
            Refusal::Window => 3,
            Refusal::Record => 4,
            Refusal::Supervisor => 5,
            Refusal::DeviceBinding => 6,
            Refusal::DeviceTrust => 7,
            Refusal::DeviceProof => 8,
            Refusal::Measurements => 9,
            Refusal::Policy => 10,
            Refusal::AppId => 11,
            Refusal::Publics => 12,
            Refusal::Body => 13,
            Refusal::Isolation(c) => 100 + c,
        }
    }
}
