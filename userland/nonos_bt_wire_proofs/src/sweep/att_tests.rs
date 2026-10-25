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
//! The ATT responses a client reads (Vol 3, Part F, 3.4): sound ones read
//! as built; one cut short, one octet long, or a list that is not whole
//! entries is refused.

use super::rng::{each, within};
use nonos_bt_wire::att::response::{error_rsp, found_ranges, handle_pairs, handle_value};

#[test]
fn att_responses_read_as_built_and_broken_lists_are_refused() {
    each(|rng| {
        let e = rng.bytes(4);
        let got = error_rsp(&e).expect("sound");
        assert_eq!(
            (got.request, got.handle, got.code),
            (e[0], u16::from_le_bytes([e[1], e[2]]), e[3])
        );
        assert!(
            error_rsp(&e[..rng.below(4)]).is_err() && error_rsp(&[&e[..], &[0]].concat()).is_err()
        );
        let n = 4 * (1 + rng.below(8));
        let ranges = rng.bytes(n);
        assert_eq!(found_ranges(&ranges).unwrap().len(), ranges.len() / 4);
        let cut = rng.below(ranges.len());
        assert_eq!(found_ranges(&ranges[..cut]).is_err(), cut % 4 != 0 || cut == 0);
        let each = 2 + rng.below(20);
        let n = each * (1 + rng.below(6));
        let list = rng.bytes(n);
        let pairs = handle_pairs(&list, each).expect("sound");
        for (k, (h, v)) in pairs.iter().enumerate() {
            assert_eq!(*h, u16::from_le_bytes([list[k * each], list[k * each + 1]]));
            assert!(v.len() == each - 2 && within(&list, v));
        }
        assert!(handle_pairs(&list[..list.len() - 1 - rng.below(each - 1)], each).is_err());
        let hv = rng.span(2, 30);
        let got = handle_value(&hv).expect("sound");
        assert_eq!((got.handle, got.value), (u16::from_le_bytes([hv[0], hv[1]]), &hv[2..]));
        assert!(handle_value(&hv[..rng.below(2)]).is_err());
    });
}
