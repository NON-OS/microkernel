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

//! The boot-long store: sixteen records, ids never reused while held.

use nonos_bt_bond::{Keys, KeysError, MemoryKeys, MEMORY_RECORDS};

#[test]
fn sixteen_records_by_kind_with_distinct_ids() {
    let mut k = MemoryKeys::default();
    let ids: Vec<u32> =
        (0..MEMORY_RECORDS).map(|i| k.store(10 + (i % 2) as u8, &[i as u8]).unwrap()).collect();
    assert_eq!(k.store(10, &[0xff]), Err(KeysError::Full));
    assert!(ids.iter().all(|&i| i != 0));
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), MEMORY_RECORDS);
    assert_eq!(k.list(10).unwrap().len(), MEMORY_RECORDS / 2);
    assert_eq!(k.retrieve(ids[3]), Ok(vec![3]));
    k.delete(ids[3]).unwrap();
    assert_eq!(k.delete(ids[3]), Err(KeysError::NotFound));
    assert_eq!(k.retrieve(ids[3]), Err(KeysError::NotFound));
    let again = k.store(12, &[7]).unwrap();
    assert!(!ids[..3].contains(&again) && !ids[4..].contains(&again));
}
