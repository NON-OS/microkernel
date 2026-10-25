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

//! Each round the AP sends a whole frame to the station and its twin, or the
//! chip lies to the station alone. A lie is refused and changes nothing.

use super::air::{frame, keyed, lie, GTK_ID, RSC};
use crate::ap_sim::GTK;
use crate::receive::ccmp_tests::tk;
use crate::sweep::rng::{Rng, SEEDS};
use nonos_wifi_core::station::{Ccmp, LinkStation, Rx, RxDrop};

/// What the station made of a frame, comparable.
fn outcome(r: Rx) -> Result<Vec<u8>, Option<RxDrop>> {
    match r {
        Rx::Ethernet(eth) => Ok(eth),
        Rx::Dropped(d) => Err(Some(d)),
        Rx::Handshake { .. } | Rx::Send(_) | Rx::Lost(_) => Err(None),
    }
}

fn both(s: &mut LinkStation, twin: &mut LinkStation, f: &[u8]) -> Result<Vec<u8>, Option<RxDrop>> {
    let a = outcome(s.receive(f, true));
    assert_eq!(a, outcome(twin.receive(f, true)), "the station and its twin differ");
    a
}

/// The sweep for stations keyed by `ccmp`; `mic` says the station checks
/// the MIC itself, so the chip can lie about that too.
pub fn sweep(ccmp: fn() -> Ccmp, mic: bool) {
    for seed in SEEDS {
        let mut rng = Rng::new(seed);
        let (mut s, mut twin) = (keyed(ccmp()), keyed(ccmp()));
        let (mut pn, mut gpn) = (0u64, RSC);
        for round in 0..400 {
            if rng.below(2) == 0 {
                let group = rng.below(3) == 0;
                let last = if group { &mut gpn } else { &mut pn };
                *last += 1 + rng.below(3) as u64;
                let (key, kid) = if group { (GTK, GTK_ID) } else { (tk(), 0) };
                let f = frame(&mut rng, group, &key, *last, kid);
                assert!(both(&mut s, &mut twin, &f).is_ok(), "seed {seed:x} round {round}");
                continue;
            }
            let (f, replay) = lie(&mut rng, pn, gpn, mic);
            let want = if replay { RxDrop::Replay } else { RxDrop::Undecryptable };
            assert_eq!(
                outcome(s.receive(&f, true)),
                Err(Some(want)),
                "seed {seed:x} round {round}"
            );
        }
        assert!(s.is_associated() && s.bssid() == twin.bssid());
        for (group, key, last, kid) in [(false, tk(), pn, 0), (true, GTK, gpn, GTK_ID)] {
            let f = frame(&mut rng, group, &key, last, kid);
            assert_eq!(both(&mut s, &mut twin, &f), Err(Some(RxDrop::Replay)));
            let f = frame(&mut rng, group, &key, last + 1, kid);
            assert!(both(&mut s, &mut twin, &f).is_ok());
        }
    }
}
