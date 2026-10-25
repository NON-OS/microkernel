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

use super::drain::drain_for_pid;
use super::records::{allocate_id, insert_if, try_each};
use super::types::DmaGrant;

fn grant(pid: u32, phys: u64) -> DmaGrant {
    DmaGrant {
        grant_id: allocate_id(),
        pid,
        device_id: 7,
        claim_epoch: 1,
        physical_start: phys,
        user_va: 0,
        length: 4096,
        flags: 0,
        device_addr: phys,
        confined: false,
    }
}

// The table is one static, so the cases share it and run as one test, each
// leaving it as it found it.
#[test]
fn the_wipe_walk_sees_every_live_grant_and_waits_on_a_held_table_only_as_long_as_bounded() {
    let pid = 0xD3A1;
    for i in 0..5u64 {
        assert!(matches!(insert_if(grant(pid, 0x10_0000 + i * 4096), |_| true), Some(Ok(()))));
    }
    let mut seen = std::vec::Vec::new();
    let first = |g: &DmaGrant| {
        if g.pid == pid {
            seen.push(g.physical_start)
        }
    };
    assert!(try_each(first, || false), "a free table is taken at the first try");
    seen.sort();
    assert_eq!(seen, (0..5).map(|i| 0x10_0000 + i * 4096).collect::<std::vec::Vec<_>>());

    // A CPU stopped by IPI while holding the table, modelled by a walk that is
    // itself running: the inner walk tries for as long as its caller's bound
    // says (L73), then reports that it could not look and visits nothing,
    // instead of spinning on a lock nobody will release.
    let mut inner_saw = None;
    let outer = |_: &DmaGrant| {
        if inner_saw.is_none() {
            let (mut visited, mut tries) = (0, 0);
            let looked = try_each(
                |_| visited += 1,
                || {
                    tries += 1;
                    tries < 4
                },
            );
            inner_saw = Some((looked, visited, tries));
        }
    };
    assert!(try_each(outer, || false));
    assert_eq!(inner_saw, Some((false, 0, 4)), "asked four times, then let go");

    assert_eq!(drain_for_pid(pid).len(), 5);
}
