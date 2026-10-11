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

use super::fixture::event;
use super::fixture::{start, METER, SETTLEMENT};
use super::host::Host;
use nonos_nox_meter::open::confirmed;
use nonos_nox_meter::MeterError;
use nonos_settle_proto::Opened;

#[test]
fn the_event_this_meter_asked_for_becomes_a_fresh_channel() {
    let c = confirmed(&Host, &start(), &METER, &SETTLEMENT, &event()).unwrap();
    assert_eq!(
        (c.id, c.open_nonce, c.unit, c.deposit),
        (event().channel_id, 4, start().unit, 1_000)
    );
    assert_eq!(
        (c.settled, c.cumulative, c.seq, c.expiry, c.frozen),
        (0, 0, 0, start().expiry, false)
    );
}

#[test]
fn any_field_off_the_request_or_an_id_off_its_nonce_is_refused() {
    let e = event();
    for bad in [
        Opened { payer: [1; 20], ..e },
        Opened { payee: [1; 20], ..e },
        Opened { listing_id: [1; 32], ..e },
        Opened { amount: 999, ..e },
        Opened { expiry: 1, ..e },
        Opened { reclaim_to: [1; 20], ..e },
        Opened { open_nonce: 5, ..e },
        Opened { channel_id: [1; 32], ..e },
    ] {
        assert_eq!(
            confirmed(&Host, &start(), &METER, &SETTLEMENT, &bad),
            Err(MeterError::Mismatch)
        );
    }
    assert!(confirmed(&Host, &start(), &METER, &[0x5f; 20], &e).is_err());
}
