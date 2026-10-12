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

//! Scalars: integers modulo the group order r.

use crate::constants::LEN;
use p256::elliptic_curve::ops::Reduce;
use p256::elliptic_curve::{Field, PrimeField};
use p256::{FieldBytes, Scalar, U256};

/// A scalar from 32 bytes, if it is below the order.
pub fn scalar_from_bytes(b: &[u8; LEN]) -> Option<Scalar> {
    Option::from(Scalar::from_repr(FieldBytes::from(*b)))
}

/// A 32-byte value reduced modulo the order.
pub fn scalar_reduce(b: &[u8; LEN]) -> Scalar {
    <Scalar as Reduce<U256>>::reduce_bytes(&FieldBytes::from(*b))
}

/// A scalar's 32-byte big-endian encoding.
pub fn scalar_bytes(s: &Scalar) -> [u8; LEN] {
    let mut out = [0u8; LEN];
    out.copy_from_slice(&s.to_bytes());
    out
}

/// Whether a scalar is 0 or 1. SAE accepts neither (1 < scalar < r), and
/// no protocol here takes 0 as a secret.
pub fn scalar_is_trivial(s: &Scalar) -> bool {
    bool::from(s.is_zero()) || *s == Scalar::ONE
}
