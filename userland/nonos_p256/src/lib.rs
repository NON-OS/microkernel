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

//! NIST P-256 for the whole system: y^2 = x^3 - 3x + b over GF(p) (FIPS
//! 186-4, D.1.2.3), IEEE 802.11 group 19 and the curve of LE Secure
//! Connections. Every protocol that runs on the curve takes it from here.
//!
//! What this crate adds over the arithmetic: 32-byte big-endian encodings
//! of field elements, scalars and points, each checked on the way in (a
//! value at or above its modulus, or a point off the curve, is refused
//! rather than reduced), and ECDH with a secret that is overwritten when
//! dropped.
//!
//! The arithmetic is RustCrypto's p256 (primeorder 0.13). Its scalar
//! multiplication is constant time in the scalar: a fixed 4-bit window run
//! over all 64 windows, a 16-entry table of multiples read in full at every
//! window with a masked select so the memory touched never depends on the
//! scalar's bits, and the complete addition and doubling formulas of
//! Renes, Costello and Batina (2015), which have no special case for the
//! identity or for equal inputs and so nothing to branch on. Field
//! inversion and square roots are fixed exponentiations.

#![no_std]

mod constants;
mod ecdh;
mod field;
mod point;
mod scalar;

pub use constants::*;
pub use ecdh::Secret;
pub use field::*;
pub use p256::elliptic_curve::bigint::Encoding;
pub use p256::elliptic_curve::group::Group;
pub use p256::elliptic_curve::subtle::{Choice, ConditionallySelectable, ConstantTimeEq};
pub use p256::elliptic_curve::{Field, PrimeField};
pub use p256::{AffinePoint, FieldElement, ProjectivePoint, Scalar, U256};
pub use point::*;
pub use scalar::*;
