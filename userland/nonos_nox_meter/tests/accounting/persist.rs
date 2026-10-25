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
use super::fixture::{funded_state, LISTING, NOW, PAYEE, ROOT};
use nonos_nox_meter::budget::ListingCap;
use nonos_nox_meter::persist::{decode, encode, CHANNEL_LEN, LISTING_LEN, STATE_LEN};
use nonos_nox_meter::MeterError;

const HEADER: usize = 72;
const CHANNELS_AT: usize = HEADER + 2 + 32 * LISTING_LEN;

pub(super) fn sample() -> nonos_nox_meter::state::MeterState {
    let (s, c) = funded_state().charged(&PAYEE, &LISTING, 1, 123, ROOT, NOW).unwrap();
    let mut s = s.signed(&c.id, c.seq, [0x5a; 65]);
    s.budget.listings.push(ListingCap { listing_id: LISTING, cap: 77, spent: 5 });
    s.channels[0].carried = 100;
    s.channels[0].reclaim_to = [0x77; 20];
    s.last_settle = Some((1, 7));
    s.last_retry = 99;
    s
}

#[test]
fn the_state_reads_back_as_written_at_one_length() {
    let s = sample();
    let b = encode(&s).unwrap();
    assert_eq!((b.len(), CHANNEL_LEN), (STATE_LEN, 330), "v5: reclaim_to");
    assert_eq!(decode(&b), Ok(s));
    let empty = encode(&Default::default()).unwrap();
    assert_eq!(empty.len(), STATE_LEN);
    assert_eq!(decode(&empty), Ok(Default::default()));
}

#[test]
fn any_other_length_is_refused() {
    let b = encode(&sample()).unwrap();
    for n in [0, 8, CHANNELS_AT, STATE_LEN - 1] {
        assert_eq!(decode(&b[..n]), Err(MeterError::Corrupt), "{n} bytes");
    }
    let mut longer = b.clone();
    longer.push(0);
    assert_eq!(decode(&longer), Err(MeterError::Corrupt));
}

#[test]
fn a_wrong_header_a_count_or_a_byte_in_an_unused_slot_is_refused() {
    let b = encode(&sample()).unwrap();
    let unused_listing = HEADER + 2 + LISTING_LEN;
    let unused_channel = CHANNELS_AT + 2 + CHANNEL_LEN;
    /* Byte 4 flipped reads version 4: the file before reclaim_to is refused. */
    for at in [0, 4, 6, unused_listing, unused_channel, STATE_LEN - 1] {
        let mut bad = b.clone();
        bad[at] ^= 1;
        assert_eq!(decode(&bad), Err(MeterError::Corrupt), "byte {at}");
    }
    for (at, n) in [(HEADER, 33u16), (CHANNELS_AT, 65)] {
        let mut many = b.clone();
        many[at..at + 2].copy_from_slice(&n.to_le_bytes());
        assert_eq!(decode(&many), Err(MeterError::Corrupt), "count at {at}");
    }
}
