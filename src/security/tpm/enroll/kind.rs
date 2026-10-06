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

//! Which of the profile's two low-range EKs, and where its certificate is.

/// The EK a caller asks for.
///
/// Prefer `EccP256` when [`super::ek_certificate`] finds its certificate: a
/// P-256 primary is derived in milliseconds where an RSA 2048 primary costs a
/// prime search of seconds on a discrete part, and its public area is a fifth
/// the size. Fall back to `Rsa2048` when the ECC index is empty: it is the
/// certificate every PC Client TPM is required to ship.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EkKind {
    /// Template L-1, certificate at `0x01C00002`.
    Rsa2048,
    /// Template L-2, certificate at `0x01C0000A`.
    EccP256,
}

impl EkKind {
    /// The NV index the manufacturer writes this EK's certificate to.
    pub const fn cert_index(self) -> u32 {
        match self {
            EkKind::Rsa2048 => 0x01C0_0002,
            EkKind::EccP256 => 0x01C0_000A,
        }
    }

    /// The sizes of the fields of `unique`: the modulus, or x then y. The
    /// template fills them with zeros and the TPM answers at these sizes.
    pub(super) const fn unique(self) -> &'static [usize] {
        match self {
            EkKind::Rsa2048 => &[256],
            EkKind::EccP256 => &[32, 32],
        }
    }
}
