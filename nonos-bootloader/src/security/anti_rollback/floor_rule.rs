// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! What the rollback floor is on this machine. The floor is a TPM monotonic
//! counter: without a TPM, or with one whose counter cannot be read, there is
//! nothing to hold an older signed kernel back. A profile that requires a TPM
//! then refuses to boot; any other boots, and says the protection is off.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Floor {
    /// The TPM's counter, which the image's rollback index must reach.
    Held(u64),
    /// No readable counter, and the profile requires one: the boot stops.
    Refuse,
    /// No readable counter, and the profile allows that: no floor, shown.
    Unprotected,
}

/// `floor` is what the TPM's counter read gave, `None` when there is no TPM or
/// the read failed.
pub fn floor_rule(floor: Option<u64>, profile_requires_tpm: bool) -> Floor {
    match floor {
        Some(f) => Floor::Held(f),
        None if profile_requires_tpm => Floor::Refuse,
        None => Floor::Unprotected,
    }
}
