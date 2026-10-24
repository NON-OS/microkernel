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

//! The second bulk pair's admission: beside the first, on endpoints the
//! first does not hold, one a slot; and the ops' pair byte.

use super::fixture::Fixture;
use super::toggle_slot::storage_slot;
use crate::controller::BulkPipes;
use crate::slots::{pair_of, pair_stream};

const DATA_IN_DCI: u8 = 6;
const DATA_OUT_DCI: u8 = 7;

#[test]
fn a_second_pair_is_taken_only_beside_the_first_on_its_own_endpoints() {
    let fx = Fixture::new();
    let mut res = storage_slot(&fx);
    assert!(!res.takes_second_bulk((DATA_IN_DCI, 5)), "the first pair's OUT");
    assert!(!res.takes_second_bulk((4, DATA_OUT_DCI)), "the first pair's IN");
    assert!(res.takes_second_bulk((DATA_IN_DCI, DATA_OUT_DCI)));
    res.bulk2 = Some(BulkPipes::new(&fx.pool, DATA_IN_DCI, DATA_OUT_DCI).expect("pipes"));
    assert!(!res.takes_second_bulk((8, 9)), "one second pair a slot");
    assert_eq!(res.pair_mut(true).map(|p| (p.dci_in, p.dci_out)), Some((6, 7)));
    assert_eq!(res.pair_mut(false).map(|p| (p.dci_in, p.dci_out)), Some((4, 5)));
    res.bulk = None;
    assert!(res.has_bulk(), "the second alone still holds the slot");
}

#[test]
fn the_pair_byte_is_0_or_1_and_older_clients_send_0() {
    assert_eq!(
        (pair_of(0), pair_of(1), pair_of(2), pair_of(0xFF)),
        (Some(false), Some(true), None, None)
    );
}

#[test]
fn bulk_ops_name_a_stream_above_the_pair_and_a_poll_never_does() {
    assert_eq!((pair_stream(0x21), pair_stream(0x10)), (Some((true, 2)), Some((false, 1))));
    assert_eq!((pair_stream(0x02), pair_stream(0xF2)), (None, None));
    assert_eq!(pair_of(0x21), None, "the poll's byte takes no stream");
}
