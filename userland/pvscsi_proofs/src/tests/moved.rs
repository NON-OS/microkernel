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


//! A completion's host status read as the emulated device writes it: the
//! data length is zero on every completion, and a short transfer is marked
//! BTSTAT_DATARUN.

use nonos_scsi::Fault;

use crate::moved::{completed, BT_DATARUN};

#[test]
fn a_normal_completion_moved_everything_whatever_its_length_says() {
    for len in [0usize, 36, 512, 65536] {
        for (host, moved) in [(0x00u16, 0u64), (0x00, 7), (0x0A, 0), (0x0B, 0)] {
            assert_eq!(completed(host, len, moved, false), Ok(0), "{host:#x} {len} {moved}");
        }
    }
}

#[test]
fn a_short_transfer_leaves_what_its_length_does_not_cover() {
    for host in [0x0Cu16, BT_DATARUN] {
        assert_eq!(completed(host, 512, 0, true), Ok(512), "a short read is short");
        assert_eq!(completed(host, 512, 200, true), Ok(312));
        assert_eq!(completed(host, 512, 9000, false), Ok(0), "never more than asked");
    }
    assert_eq!(completed(0x0C, 512, 0, false), Ok(512), "an underrun says its length");
    assert_eq!(completed(BT_DATARUN, 512, 0, false), Ok(0), "the answer's own length rules");
}

#[test]
fn the_faults_are_told_apart() {
    assert_eq!(completed(0x11, 36, 0, false), Err(Fault::NoTarget));
    for host in [0x13u16, 0x22, 0x23, 0x25, 0x15, 0x16] {
        let f = completed(host, 36, 0, false);
        assert!(matches!(f, Err(Fault::Reset) | Err(Fault::Failed)), "{host:#x}: {f:?}");
    }
    assert_eq!(completed(0x1A, 36, 0, false), Err(Fault::Failed));
}
