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

use super::behaviour::requires_write_buffer_flush;
use super::global::{gcmd_with, GCMD_ONE_SHOT, GCMD_SRTP, GCMD_TE, GCMD_WBF, GSTS_RTPS, GSTS_TES};

/// The mask the VT-d specification prescribes (10.4.4): one-shot bits clear.
const SPEC_PERSISTENT_MASK: u32 = 0x96FF_FFFF;

#[test]
fn the_one_shot_set_is_the_specification_mask() {
    assert_eq!(!GCMD_ONE_SHOT, SPEC_PERSISTENT_MASK);
}

#[test]
fn turning_translation_on_does_not_set_the_root_pointer_again() {
    // After SRTP completes, status reports RTPS. The old code carried that
    // into the TE write, so enabling translation issued SRTP a second time.
    let status = GSTS_RTPS;
    let command = gcmd_with(status, GCMD_TE);
    assert_eq!(command & GCMD_SRTP, 0, "TE write repeated SRTP");
    assert_ne!(command & GCMD_TE, 0);
    assert_eq!(status | GCMD_TE, GCMD_SRTP | GCMD_TE, "the old construction, for the record");
}

#[test]
fn every_status_word_keeps_its_controls_and_drops_its_one_shots() {
    for bit in 0..32u32 {
        let status = 1u32 << bit;
        let command = gcmd_with(status, 0);
        if GCMD_ONE_SHOT & status != 0 {
            assert_eq!(command, 0, "one-shot bit {bit} carried over");
        } else {
            assert_eq!(command, status, "persistent bit {bit} lost");
        }
    }
    let mut s = 0x2545_F491_4F6C_DD1Du64;
    for _ in 0..100_000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let status = s as u32;
        for command in [GCMD_TE, GCMD_WBF, GCMD_SRTP, 0] {
            let w = gcmd_with(status, command);
            assert_eq!(w & !command & GCMD_ONE_SHOT, 0);
            assert_eq!(w & SPEC_PERSISTENT_MASK & !command, status & SPEC_PERSISTENT_MASK & !command);
            assert_eq!(w & command, command);
        }
    }
}

#[test]
fn a_write_buffer_flush_keeps_translation_on() {
    let status = GSTS_TES | GSTS_RTPS;
    let command = gcmd_with(status, GCMD_WBF);
    assert_ne!(command & GCMD_TE, 0, "flushing the write buffer turned translation off");
    assert_eq!(command & GCMD_SRTP, 0);
    assert_ne!(command & GCMD_WBF, 0);
}

#[test]
fn rwbf_is_capability_bit_four() {
    assert!(requires_write_buffer_flush(1 << 4));
    assert!(!requires_write_buffer_flush(!(1u64 << 4)));
}
