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

mod build;
mod confirm;
mod types;
use super::group::{point_xy, scalar_bytes, scalar_from_bytes, scalar_is_trivial, LEN};
use crate::wpa::kdf::kdf_sha256;
use crate::wpa::sha256::{hmac_sha256, hmac_sha256_parts};
pub use confirm::*;
use p256::{ProjectivePoint, Scalar};
pub use types::*;
