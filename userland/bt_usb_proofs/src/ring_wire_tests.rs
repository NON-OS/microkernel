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

//! The bytes the client sends, field by field as the keyring's handlers
//! read them, and its reading of the answers.

use crate::ring::answer::{id, ids, refusal};
use crate::ring_sim::{capsule, ring};
use crate::ring_tests::record;
use nonos_bt_bond::{Keys, KeysError, KIND_BOND, KIND_LOCAL_IRK};

#[test]
fn each_request_is_laid_out_as_the_keyring_reads_it() {
    let ring = ring(true);
    let mut keys = capsule(&ring, 0x0102_0304);
    let bond = record(3).encode().unwrap();
    let id = keys.store(KIND_BOND, &bond).unwrap();
    keys.list(KIND_LOCAL_IRK).unwrap();
    keys.retrieve(id).unwrap();
    keys.delete(id).unwrap();
    let r = ring.borrow();
    let pid = [4, 3, 2, 1];
    let mut store = [&pid[..], &4242u64.to_le_bytes(), &[0; 8], &[10, 48, 0]].concat();
    store.extend_from_slice(&bond);
    let by_id = [&pid[..], &id.to_le_bytes()].concat();
    let expected = [(1, store), (30, [&pid[..], &[11]].concat()), (2, by_id.clone()), (3, by_id)];
    assert_eq!(r.requests, expected);
}

#[test]
fn answers_that_do_not_add_up_are_refused() {
    assert_eq!(id(&[7, 0, 0, 0]), Ok(7));
    for bad in [&[7, 0, 0][..], &[7, 0, 0, 0, 0]] {
        assert_eq!(id(bad), Err(KeysError::Unavailable));
    }
    assert_eq!(ids(&[0, 0, 0, 0]), Ok(vec![]));
    assert_eq!(ids(&[2, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0]), Ok(vec![5, 6]));
    let short = [&[2, 0, 0, 0, 5, 0, 0, 0][..], &[1, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0], &[1, 0, 0]];
    for bad in short {
        assert_eq!(ids(bad), Err(KeysError::Unavailable), "{bad:?}");
    }
    assert_eq!(ids(&[1, 0, 0, 0, 5, 0, 0, 0, 6]), Err(KeysError::Unavailable));
}

#[test]
fn errnos_are_named_and_the_rest_mean_no_keyring() {
    assert_eq!(refusal(-2), KeysError::NotFound);
    assert_eq!(refusal(-13), KeysError::Denied);
    assert_eq!(refusal(-28), KeysError::Full);
    for other in [-22, -16, -11, -1] {
        assert_eq!(refusal(other), KeysError::Unavailable);
    }
}
