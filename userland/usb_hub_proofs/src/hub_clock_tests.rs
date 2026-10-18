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

//! Bringing up a hub's ports never sleeps: a new hub's ports are first
//! looked at once their power is good, a deadline; and a hub whose ports
//! flap is looked at for a minute with no port reset more than its budget
//! allows. The walk slept through the power-on time, up to 510 ms, and
//! through a debounce and a reset on every port at every pass.

use nonos_usb_hub::{scan_hubs, Hub, SPEED_HIGH, TRIES_PER_MINUTE};

use crate::fake::{root_hub, Bind, Device, Fake, HubModel, Kind};

fn hub_with(h: &mut Fake, ports: usize) -> Vec<Hub> {
    let mut model = HubModel::new(false, ports);
    for p in 1..=ports {
        model = model.plug(p, Device { speed: SPEED_HIGH, kind: Kind::Ignore });
    }
    vec![root_hub(h, model, 1, SPEED_HIGH)]
}

#[test]
fn a_new_hubs_ports_wait_for_power_good_on_the_clock() {
    let mut h = Fake::default();
    let mut hubs = hub_with(&mut h, 1);
    let ready = hubs[0].ready_ms as i64;
    assert!(ready >= h.now + 100, "the descriptor's 100 ms is a deadline: {ready}");
    let (mut c, mut first_reset) = (Bind::default(), None);
    while h.now < ready + 500 && first_reset.is_none() {
        scan_hubs(&mut h, &mut hubs, &mut c, false);
        first_reset = (!h.resets.is_empty()).then_some(h.now);
        h.now += 1;
    }
    assert_eq!(first_reset, Some(ready + 100), "power good, then 100 ms of debounce");
    assert_eq!(h.slept, 0, "never asleep");
}

#[test]
fn a_flapping_hub_is_tried_a_few_times_a_minute_and_never_sleeps() {
    let mut h = Fake::default();
    let mut hubs = hub_with(&mut h, 4);
    let c = &mut Bind::default();
    let start = h.now;
    while h.now < start + 60_000 {
        let up = (h.now - start) % 200 < 150;
        for port in 1..=4 {
            let p = h.port(1, port);
            if p.device.is_some() != up {
                p.device = up.then_some(Device { speed: SPEED_HIGH, kind: Kind::Ignore });
                (p.c_connection, p.enabled) = (true, false);
            }
        }
        scan_hubs(&mut h, &mut hubs, c, false);
        h.now += 10;
    }
    for port in 1..=4 {
        let n = h.resets.iter().filter(|r| r.1 == port).count();
        assert!((1..=usize::from(TRIES_PER_MINUTE)).contains(&n), "port {port}: {n} resets");
    }
    assert_eq!(h.slept, 0, "never asleep");
}
