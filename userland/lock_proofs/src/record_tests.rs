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

//! The record's bytes.

use nonos_lock::argon2::Params;
use nonos_lock::record::{runnable, MAGIC, PARAMS};
use nonos_lock::{Record, RECORD_LEN};

fn sample() -> Record {
    Record { params: PARAMS, salt: [7u8; 16], tag: [9u8; 32], failures: 5 }
}

#[test]
fn a_record_reads_back_as_written() {
    let bytes = sample().encode();
    assert_eq!(bytes.len(), RECORD_LEN);
    assert_eq!(RECORD_LEN, 62);
    assert_eq!(&bytes[..4], &MAGIC);
    let back = Record::decode(&bytes).unwrap();
    assert_eq!(back.params, PARAMS);
    assert_eq!(back.salt, [7u8; 16]);
    assert_eq!(back.tag, [9u8; 32]);
    assert_eq!(back.failures, 5);
}

#[test]
fn a_wrong_length_or_magic_is_no_record() {
    let bytes = sample().encode();
    assert!(Record::decode(&bytes[..RECORD_LEN - 1]).is_none());
    let mut longer = bytes.to_vec();
    longer.push(0);
    assert!(Record::decode(&longer).is_none());
    let mut magic = bytes;
    magic[0] ^= 1;
    assert!(Record::decode(&magic).is_none());
}

#[test]
fn parameters_the_keyring_cannot_run_are_refused() {
    for p in [
        Params { m_kib: PARAMS.m_kib + 1, t: 3, p: 4 },
        Params { m_kib: 4096, t: 0, p: 1 },
        Params { m_kib: 4096, t: 17, p: 1 },
        Params { m_kib: 4096, t: 1, p: 0 },
        Params { m_kib: 8, t: 1, p: 2 },
    ] {
        assert!(!runnable(p), "{p:?}");
        let mut r = sample();
        r.params = p;
        assert!(Record::decode(&r.encode()).is_none(), "{p:?}");
    }
    assert!(runnable(PARAMS));
    assert!(runnable(Params { m_kib: 16, t: 1, p: 2 }));
}
