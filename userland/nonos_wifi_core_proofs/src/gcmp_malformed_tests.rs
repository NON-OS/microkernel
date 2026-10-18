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

//! GCMP-256 framing refuses a frame too short for its MIC, one whose key-id
//! octet lacks ExtIV, and one too short for its own header.

use super::gcmp_frame_tests::{data, TK};
use nonos_wifi_core::dot11::gcmp::{decrypt, encrypt, GCMP_HDR_LEN, GCMP_MIC_LEN};

#[test]
fn malformed_frames_are_refused() {
    let p = encrypt(&data(false), 1, &TK).unwrap();
    assert_eq!(decrypt(&p[..24 + GCMP_HDR_LEN + GCMP_MIC_LEN - 1], &TK), None);
    let mut q = p.clone();
    q[24 + 3] &= !0x20;
    assert_eq!(decrypt(&q, &TK), None);
    assert_eq!(decrypt(&p[..10], &TK), None);
    assert_eq!(encrypt(&[0x08, 0x01, 0, 0], 1, &TK), None);
}
