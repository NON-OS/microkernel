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

//! What a failed roam reports. It is never "wrong password": the session's
//! secret already joined this network once, so a candidate that fails it
//! has proved nothing about the password, and a rogue AP must not be able
//! to make the person doubt a password that works.

use crate::softmac::join::Report;

/// The candidate accepted the move, then did not prove it holds the key:
/// the handshake failed, or it left the station before keying.
pub const CODE_AP_UNPROVEN: i32 = -13;
/// The candidate refused the move or never answered it, an SAE confirm
/// that failed included.
pub const CODE_CANDIDATE_REFUSED: i32 = -14;

pub fn roam_code(report: &Report) -> i32 {
    if report.accepted {
        CODE_AP_UNPROVEN
    } else {
        CODE_CANDIDATE_REFUSED
    }
}
