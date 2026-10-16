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

//! The driver's one unasked message: its envelope byte for byte, its
//! reasons read back, and nothing else read as one.

use crate::notice::{notice, why, LEAVING, NCMP_MAGIC, NOTICE_LEN, OUTPUTS_CHANGED};

#[test]
fn the_notice_is_the_ncmp_envelope_and_one_reason() {
    let n = notice(OUTPUTS_CHANGED);
    assert_eq!(n.len(), NOTICE_LEN);
    assert_eq!(n[..4], NCMP_MAGIC.to_le_bytes());
    assert_eq!((n[4], n[6], n[7]), (1, 0x12, 0), "version 1, op 0x0012");
    assert_eq!(n[8..16], [0; 8], "no flags, request id 0");
    assert_eq!(n[16..20], 4u32.to_le_bytes(), "a four-byte body");
    assert_eq!(why(&n[20..]), Some(OUTPUTS_CHANGED));
    assert_eq!(why(&notice(LEAVING)[20..]), Some(LEAVING));
}

#[test]
fn nothing_else_reads_as_a_reason() {
    for w in [0u32, 3, u32::MAX] {
        assert_eq!(why(&w.to_le_bytes()), None, "{w}");
    }
    assert_eq!(why(&[1, 0, 0]), None, "short");
    assert_eq!(why(&[1, 0, 0, 0, 0]), None, "long");
}
