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

//! The sector-to-LBA map the wire client shares with NVMe's (lba_map.rs):
//! a namespace's size in sectors, a run of whole LBAs cut into requests no
//! larger than the driver takes, and that size from MDTS.

use super::lba_map::{capacity_sectors, chunks, lbas_per_request, max_transfer_bytes, plan};

#[test]
fn a_namespace_is_counted_in_sectors_or_refused() {
    assert_eq!(capacity_sectors(1000, 4096), Some(8000));
    assert_eq!(capacity_sectors(1000, 512), Some(1000));
    assert_eq!(capacity_sectors(u64::MAX, 4096), None, "too large to count");
}

#[test]
fn whole_lbas_go_in_requests_of_at_most_the_drivers_size() {
    let whole = plan(0, 10 * 4096, 4096).unwrap().body.unwrap();
    let got: Vec<_> = chunks(whole, 4, 4096).collect();
    assert_eq!(got, [(0, 4, 0), (4, 4, 4 * 4096), (8, 2, 8 * 4096)]);
    assert_eq!(chunks(whole, 0, 4096).count(), 10, "a size of none moves one at a time");
}

#[test]
fn mdts_limits_a_request_and_no_mdts_leaves_the_buffer() {
    assert_eq!(max_transfer_bytes(0, 128 * 1024), 128 * 1024, "zero is no limit");
    assert_eq!(max_transfer_bytes(1, 128 * 1024), 8192, "two 4 KiB pages");
    assert_eq!(max_transfer_bytes(63, 4096), 4096, "too large to shift");
    assert_eq!(lbas_per_request(1, 4096, 128 * 1024), 2);
    assert_eq!(lbas_per_request(0, 512, 128 * 1024), 256);
}
