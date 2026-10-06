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

use super::log_build::{digest, ev, header, standard, EV_APP, SHA1, SHA256, SHA384};
use crate::tcg::{replay, LogError, MAX_ALGS};

fn log_of(events: &[Vec<u8>]) -> Vec<u8> {
    let mut log = standard();
    events.iter().for_each(|e| log.extend(e));
    log
}

#[test]
fn a_cut_inside_an_event_or_a_byte_past_the_last_is_refused() {
    let log = log_of(&[ev(4, EV_APP, digest(1)), ev(4, EV_APP, digest(2))]);
    let (head, one) = (standard().len(), ev(4, EV_APP, digest(1)).len());
    for cut in 0..log.len() {
        let r = replay(&log[..cut]);
        if cut == head || cut == head + one {
            assert!(r.is_ok(), "an event boundary at {cut} is a shorter log, not a broken one");
        } else {
            assert_eq!(r, Err(LogError::Truncated), "cut at {cut}");
        }
    }
    let mut long = log.clone();
    long.push(0);
    assert_eq!(replay(&long), Err(LogError::Truncated));
}

#[test]
fn a_header_that_is_not_crypto_agile_or_has_no_sha256_is_refused() {
    let mut h = standard();
    h[4] = 1;
    assert_eq!(replay(&h), Err(LogError::NotCryptoAgile));
    let mut h = standard();
    h[32] ^= 1;
    assert_eq!(replay(&h), Err(LogError::NotCryptoAgile));
    assert_eq!(replay(&header(&[(SHA1, 20)])), Err(LogError::NoSha256Bank));
    assert_eq!(replay(&header(&[(SHA1, 20), (SHA256, 20)])), Err(LogError::NoSha256Bank));
    assert_eq!(replay(&header(&[(SHA256, 32), (SHA256, 32)])), Err(LogError::UnknownBank));
    assert_eq!(replay(&header(&[(SHA256, 32), (SHA384, 65)])), Err(LogError::UnknownBank));
    let many: Vec<(u16, u16)> = (0..=MAX_ALGS as u16).map(|a| (a + 0x20, 32)).collect();
    assert_eq!(replay(&header(&many)), Err(LogError::TooManyBanks));
}
