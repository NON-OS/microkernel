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
//! HCI events (Vol 4, Part E, 5.4.4 and 7.7): a sound event reads as built;
//! a parameter length that lies either way, or a fixed-size event one
//! octet off, is refused.

use super::rng::{each, within};
use nonos_bt_wire::hci::le_meta::{LeConnectionComplete, LeLtkRequest};
use nonos_bt_wire::hci::{parse_event, CommandComplete, CommandStatus};
use nonos_bt_wire::hci::{DisconnectionComplete, EncryptionChange};

#[test]
fn events_read_as_built_and_a_lying_length_is_refused() {
    each(|rng| {
        let (code, params) = (rng.byte(), rng.span(0, 256));
        let f = [&[code, params.len() as u8][..], &params].concat();
        let e = parse_event(&f).expect("sound");
        assert_eq!((e.code, e.params), (code, &params[..]));
        assert!(within(&f, e.params));
        let mut lie = f.clone();
        lie[1] = loop {
            let n = rng.byte();
            if usize::from(n) != params.len() {
                break n;
            }
        };
        assert!(parse_event(&lie).is_err(), "a stated length of {}", lie[1]);
        assert!(parse_event(&f[..rng.below(f.len())]).is_err(), "cut short");
    });
}

#[test]
fn fixed_events_take_exactly_their_size() {
    type Fits = fn(&[u8]) -> bool;
    let sized: [(usize, Fits); 6] = [
        (4, |p| CommandStatus::parse(p).is_ok()),
        (4, |p| DisconnectionComplete::parse(p).is_ok()),
        (4, |p| EncryptionChange::parse(p).is_ok()),
        (12, |p| LeLtkRequest::parse(p).is_ok()),
        (18, |p| LeConnectionComplete::parse(p).is_ok()),
        (3, |p| CommandComplete::parse(p).is_ok()),
    ];
    each(|rng| {
        for (i, (size, fits)) in sized.iter().enumerate() {
            let mut p = rng.bytes(*size);
            if i == 4 {
                p[4] &= 1;
            }
            assert!(fits(&p), "a sound {size}-octet event");
            assert!(!fits(&p[..rng.below(*size)]), "short of {size}");
            let long = [&p[..], &rng.span(1, 8)].concat();
            assert_eq!(fits(&long), i == 5, "past {size}: only Command Complete has a tail");
        }
        let mut c = rng.bytes(18);
        c[4] = 2 + rng.below(254) as u8;
        assert!(LeConnectionComplete::parse(&c).is_err(), "peer address type {}", c[4]);
    });
}
