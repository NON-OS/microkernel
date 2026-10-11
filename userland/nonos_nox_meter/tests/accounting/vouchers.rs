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
use super::fixture::{channel, funded_state, LISTING, NOW, PAYEE, ROOT};
use nonos_nox_meter::channel::{Channel, CHARGE_MARGIN, MAX_CHANNELS, NO_SIG};
use nonos_nox_meter::state::MeterState;
use nonos_nox_meter::MeterError;

const METER: [u8; 20] = [0xaa; 20];

#[test]
fn a_voucher_carries_its_channel_expiry_and_newest_numbers() {
    let c = channel().charged(40, ROOT, NOW).unwrap();
    let v = c.voucher(&METER);
    assert_eq!((v.expiry, v.cumulative, v.seq, v.usage_root), (c.expiry, 40, 1, ROOT));
    assert_eq!((v.channel_id, v.payer, v.payee, v.unit), (c.id, METER, c.payee, c.unit));
    assert!(c.is_newest(&v));
    for other in [v.expiry + 1, v.expiry - 1] {
        assert!(!c.is_newest(&nonos_nox_meter::voucher::Voucher { expiry: other, ..v }));
    }
}

#[test]
fn only_the_newest_voucher_takes_a_signature_and_a_charge_clears_it() {
    let (s, c) = funded_state().charged(&PAYEE, &LISTING, 1, 10, ROOT, NOW).unwrap();
    assert_eq!(s.channels[0].signed(&METER), None);
    assert_eq!(s.signed(&c.id, c.seq + 1, [7; 65]), s);
    let s = s.signed(&c.id, c.seq, [7; 65]);
    assert_eq!(s.channels[0].signed(&METER), Some((c.voucher(&METER), [7; 65])));
    let (s, _) = s.charged(&PAYEE, &LISTING, 1, 10, ROOT, NOW).unwrap();
    assert_eq!(s.channels[0].sig, NO_SIG);
}

#[test]
fn a_charge_lands_on_the_live_channel_that_lasts_longest() {
    let old = Channel { id: [1; 32], expiry: NOW, ..channel() };
    let long = Channel { id: [2; 32], open_nonce: 2, expiry: NOW + 3 * CHARGE_MARGIN, ..channel() };
    let s = funded_state().with_channel(old, NOW).unwrap().with_channel(long, NOW).unwrap();
    let (_, c) = s.charged(&PAYEE, &LISTING, 1, 5, ROOT, NOW).unwrap();
    assert_eq!(c.id, long.id);
}

#[test]
fn a_full_table_forgets_only_a_dead_and_settled_channel() {
    let mut s = MeterState { channels: vec![], ..funded_state() };
    for i in 0..MAX_CHANNELS {
        let c = Channel { id: [100 + i as u8; 32], cumulative: 5, ..channel() };
        s = s.with_channel(c, NOW).unwrap();
    }
    let extra = Channel { id: [0xff; 32], ..channel() };
    let late = NOW + 3 * CHARGE_MARGIN;
    assert_eq!(s.with_channel(extra, NOW).err(), Some(MeterError::Full));
    // Past expiry, but each still holds spend the chain has not settled.
    assert_eq!(s.with_channel(extra, late).err(), Some(MeterError::Full));
    let settled = s.synced(&[101; 32], 1000, 5);
    let s = settled.with_channel(extra, late).unwrap();
    assert_eq!(s.channels.len(), MAX_CHANNELS);
    assert!(s.channels.iter().all(|c| c.id != [101; 32]));
}
