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

//! The I/O queue pair has 8 entries; CAP.MQES (0's based) says how many a
//! controller's I/O queues may hold. One that allows fewer is refused before
//! a page is mapped or a Create command sent; 8 or more is served.

use nonos_libc::calls;

use crate::error::NvmeError;
use crate::fake::{info, up};
use crate::nvm::{bring_up, NamespaceGeometry};

const G: NamespaceGeometry =
    NamespaceGeometry { nsid: 1, capacity_sectors: 1 << 20, lba_size: 512, max_sectors: 64 };

#[test]
fn a_controller_whose_queues_hold_fewer_than_eight_is_refused_untouched() {
    let mut up = up();
    for mqes in [0u64, 1, 6] {
        let mut small = info();
        small.cap = (small.cap & !0xffff) | mqes;
        let (asked, mapped) = (up.admin_count(), calls().len());
        let got = bring_up(7, 1, up.regs, small, &mut up.admin, G).err();
        assert_eq!(got, Some(NvmeError::QueueTooShallow), "MQES {mqes}");
        assert_eq!((up.admin_count(), calls().len()), (asked, mapped), "MQES {mqes}");
    }
}

#[test]
fn a_controller_whose_queues_hold_eight_is_served() {
    let mut up = up();
    let mut exact = info();
    exact.cap = (exact.cap & !0xffff) | 7;
    let from = up.admin_count();
    assert!(bring_up(7, 1, up.regs, exact, &mut up.admin, G).is_ok());
    assert_eq!(up.ops_since(from), [0x05, 0x01], "Create I/O CQ, then SQ");
}
