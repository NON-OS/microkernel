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

//! P-256, from the kernel's own field, scalar and point arithmetic,
//! compiled from the kernel source unchanged (the same arithmetic the
//! kernel checks its boot-root record with), and an ECDSA verification
//! made over it here. Everything verified is public, so nothing here needs
//! to run in constant time. The arithmetic is exported whole: the proof
//! crates sign quotes of their own with it.

#[path = "../../../../src/crypto/asymmetric/p256/constants.rs"]
pub mod constants;
#[path = "../../../../src/crypto/asymmetric/p256/field/mod.rs"]
pub mod field;
#[path = "../../../../src/crypto/asymmetric/p256/point/mod.rs"]
pub mod point;
#[path = "../../../../src/crypto/asymmetric/p256/scalar/mod.rs"]
pub mod scalar;
mod verify;

pub(crate) use constants::{P256_A, P256_B, P256_GX, P256_GY, P256_N, P256_P};
pub use field::FieldElement;
pub use point::{AffinePoint, ProjectivePoint};
pub use scalar::Scalar;
pub use verify::verify;
