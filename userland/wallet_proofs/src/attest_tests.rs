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

//! The ReleaseAttest log (plan/31, E15): one attest per release, ever.

use crate::attest::{file::RECORD_LEN, *};

fn attest(release: u16) -> Attest {
    let mut release_id = [7; 32];
    release_id[..2].copy_from_slice(&release.to_le_bytes());
    Attest {
        chain_id: 11_155_111,
        listing_id: [1; 32],
        release_id,
        bond_id: [3; 32],
        chunk_root: [4; 32],
        content_len: 1_000,
        package_hash: [0; 32],
    }
}

#[test]
fn a_release_is_attested_once_and_never_differently() {
    let a = attest(2);
    assert_eq!(decide(&[], &a), Ok(Decision::Sign));
    let log = signed(&[], &a, [9; 65]).unwrap();
    assert_eq!(decide(&log, &a), Ok(Decision::Again([9; 65])), "the same one: its signature");
    assert_eq!(signed(&log, &a, [8; 65]).unwrap(), log, "nothing recorded twice");
    let moved = Attest { content_len: 1_001, ..a };
    assert_eq!(decide(&log, &moved), Err(Refusal::Conflict));
    let other_chain = Attest { chain_id: 1, ..moved };
    assert_eq!(decide(&log, &other_chain), Ok(Decision::Sign), "another chain is another key");
    assert_eq!(decide(&log, &attest(3)), Ok(Decision::Sign), "another release");
}

#[test]
fn a_full_log_refuses_a_new_release_but_answers_an_old_one() {
    let mut log = Vec::new();
    for i in 0..1024 {
        log = signed(&log, &attest(i as u16), [i as u8 | 1; 65]).unwrap();
    }
    assert_eq!(decide(&log, &attest(5000)), Err(Refusal::Full));
    assert_eq!(decide(&log, &attest(5)), Ok(Decision::Again([5; 65])));
}

#[test]
fn the_file_reads_back_at_one_length_and_nothing_else_reads() {
    let log = signed(&signed(&[], &attest(2), [9; 65]).unwrap(), &attest(3), [7; 65]).unwrap();
    let b = encode(&log).unwrap();
    assert_eq!((b.len(), RECORD_LEN), (FILE_LEN, 241));
    assert_eq!(decode(&b), Some(log.clone()));
    assert_eq!(decode(&encode(&[]).unwrap()), Some(vec![]));
    for at in [0, 4, 6, 8 + 2 * RECORD_LEN, FILE_LEN - 1] {
        let mut bad = b.clone();
        bad[at] ^= 1;
        assert_eq!(decode(&bad), None, "byte {at}");
    }
    assert_eq!(decode(&b[..FILE_LEN - 1]), None);
    let mut many = b.clone();
    many[6..8].copy_from_slice(&1025u16.to_le_bytes());
    assert_eq!(decode(&many), None);
}
