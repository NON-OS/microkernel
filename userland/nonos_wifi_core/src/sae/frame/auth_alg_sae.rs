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

//! The SAE fields of an Authentication frame body (IEEE Std 802.11-2020,
//! 9.3.3.12, Table 9-41): after the algorithm (3), transaction sequence and
//! status code, a commit carries the finite cyclic group, an anti-clogging
//! token when the AP asked for one, the scalar and the element; a confirm
//! carries the send-confirm counter and the confirm value. With
//! hash-to-element the token rides in an Anti-Clogging Token Container
//! element after the element instead. Parsing is bounded and refuses a group
//! other than 19, a scalar outside (1, r) and an element that is not a point.

use super::*;

/// The SAE authentication algorithm number.
pub const AUTH_ALG_SAE: u16 = 3;

/// Transaction sequence numbers.
pub const SEQ_COMMIT: u16 = 1;

pub const SEQ_CONFIRM: u16 = 2;

/// Status codes a commit carries or answers with (Table 9-50).
pub const STATUS_SUCCESS: u16 = 0;

pub const STATUS_ANTI_CLOGGING_TOKEN_REQ: u16 = 76;

pub const STATUS_GROUP_NOT_SUPPORTED: u16 = 77;

pub const STATUS_SAE_HASH_TO_ELEMENT: u16 = 126;

/// The longest anti-clogging token kept: the token is opaque to the station,
/// and this bound is well above what an AP needs while capping what is held.
pub const TOKEN_MAX: usize = 64;

/// Element ids used inside SAE frames.
pub(super) const EID_EXTENSION: u8 = 255;

pub(super) const EID_EXT_ANTI_CLOGGING_TOKEN: u8 = 93;

/// A peer's commit, validated.
#[derive(Clone, Copy)]
pub struct PeerCommit {
    pub scalar: Scalar,
    pub scalar_bytes: [u8; LEN],
    pub element: ProjectivePoint,
    pub element_bytes: [u8; 2 * LEN],
}
