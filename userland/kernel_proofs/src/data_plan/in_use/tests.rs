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

use super::installer::{in_use, parse_in_use};
use super::kernel::{in_use_record, IN_USE_LEN};

/// The disks a laptop's installer finds: (backend code, instance).
const FOUND: [(u8, u8); 4] = [(3, 0), (3, 1), (2, 0), (1, 0)];

#[test]
fn both_sides_number_the_backends_alike() {
    use super::{installer as i, kernel as k};
    assert_eq!([k::VIRTIO_BLK, k::AHCI, k::NVME, k::USB_MSC], [1, 2, 3, 4]);
    assert_eq!([i::VIRTIO_BLK, i::AHCI, i::NVME, i::USB_MSC], [1, 2, 3, 4]);
    assert_eq!(IN_USE_LEN, 4);
}

#[test]
fn the_chosen_disk_is_withheld_and_the_others_are_offered() {
    for (code, instance) in FOUND {
        let chosen = parse_in_use(&in_use_record(code, instance));
        assert_eq!(chosen, Some((code, instance)));
        let withheld: Vec<_> = FOUND.iter().filter(|&&(c, n)| in_use(chosen, c, n)).collect();
        assert_eq!(withheld, [&(code, instance)], "only the chosen disk");
    }
}

#[test]
fn no_record_or_an_unknown_one_withholds_nothing() {
    for record in [&[][..], &[3][..], &[0, 0, 0, 0][..], &[9, 0, 0, 0][..]] {
        let chosen = parse_in_use(record);
        assert_eq!(chosen, None, "{record:?}");
        assert!(FOUND.iter().all(|&(c, n)| !in_use(chosen, c, n)));
    }
}
