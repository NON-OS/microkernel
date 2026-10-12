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

mod derive_pt;
mod pwd_value_len;
use super::group::{
    curve_a, curve_b, curve_rhs, fe_bytes, fe_reduce_wide, point_from_xy, scalar_from_bytes, LEN,
    ORDER,
};
use crate::wpa::hkdf;
use crate::wpa::sha256::hmac_sha256_parts;
pub use derive_pt::*;
use p256::elliptic_curve::bigint::Encoding;
use p256::elliptic_curve::subtle::ConditionallySelectable;
use p256::{FieldElement, ProjectivePoint, U256};
pub use pwd_value_len::*;
