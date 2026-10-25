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

//! IEEE 802.11 group 19 is NIST P-256. The curve, its encodings and their
//! checks are the system's one, in nonos_p256; SAE adds only the group's
//! number.

mod number;

pub use nonos_p256::{
    curve_a, curve_b, curve_rhs, fe_bytes, fe_const, fe_from_bytes_ct, fe_reduce32, fe_reduce_wide,
    point_from_xy, point_xy, scalar_bytes, scalar_from_bytes, scalar_is_trivial, scalar_reduce,
    CURVE_B, LEN, ORDER, PRIME, TWO_256_MOD_P,
};
pub use number::GROUP_19;
