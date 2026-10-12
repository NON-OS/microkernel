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

//! A storming pin masked alone and said once, the others still delivered,
//! the window starting afresh after a quiet second, and the pin unmasked
//! again only when its owner asks.

use super::amd_model::bank;
use nonos_gpio::amd_bits::INTERRUPT_DELIVER;
use nonos_gpio::storm::{LIMIT, WINDOW_MS};
use nonos_gpio::{Delivery, Owner, Polarity, Router, Trigger};

const EDGE: Trigger = Trigger { edge: true, polarity: Polarity::Low };

#[test]
fn one_pin_masked_the_rest_delivered() {
    let (mut b, mut r) = (bank(), Router::default());
    r.bind(&mut b, 3, Owner::Device(7), EDGE).unwrap();
    r.bind(&mut b, 4, Owner::Device(8), EDGE).unwrap();
    for i in 0..LIMIT {
        b.window.raise(3);
        assert_eq!(r.interrupt(&mut b, u64::from(i)), [Delivery::Holder { device: 7, pin: 3 }]);
    }
    b.window.raise(3);
    b.window.raise(4);
    let got = r.interrupt(&mut b, 600);
    assert_eq!(
        got,
        [
            Delivery::Stormed { pin: 3, owner: Owner::Device(7) },
            Delivery::Holder { device: 8, pin: 4 }
        ]
    );
    assert_eq!(b.window.pin(3) & INTERRUPT_DELIVER, 0, "masked at the pin");
    b.window.raise(3);
    assert!(r.interrupt(&mut b, 700).is_empty(), "said once, then silent");
    r.rearm(&mut b, 3).unwrap();
    b.window.raise(3);
    assert_eq!(r.interrupt(&mut b, 800), [Delivery::Holder { device: 7, pin: 3 }]);
}

#[test]
fn a_quiet_window_starts_afresh() {
    let (mut b, mut r) = (bank(), Router::default());
    r.bind(&mut b, 3, Owner::Aei, EDGE).unwrap();
    for i in 0..(LIMIT * 3) {
        b.window.raise(3);
        let now = u64::from(i) * (WINDOW_MS / u64::from(LIMIT) + 1);
        assert_eq!(r.interrupt(&mut b, now), [Delivery::Acpi { pin: 3 }], "event {i}");
    }
}
