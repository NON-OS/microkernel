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
//! No lie in an SAE exchange gives a PMK: a commit or a confirm with one
//! octet changed leaves the station without one, while the same exchange
//! told honestly agrees.

use super::rng::{Rng, SEEDS};
use super::sae_sim::station;
use super::sae_tests::ROUNDS;
use nonos_wifi_core::sae::{SaeState, SaeStep};

fn flip(rng: &mut Rng, b: &[u8]) -> Vec<u8> {
    let mut out = b.to_vec();
    let at = rng.below(out.len());
    out[at] ^= 1 + rng.below(255) as u8;
    out
}

#[test]
fn no_lied_commit_or_confirm_gives_a_pmk() {
    for seed in SEEDS {
        let mut rng = Rng::new(seed);
        for _ in 0..ROUNDS / 2 {
            // The AP draws first: a station seeded from the next draw
            // repeats what follows it, and would only reflect the AP.
            let mut ap = station(&mut rng);
            let seed = rng.next();
            let (mut sta, mut twin) = (station(&mut Rng::new(seed)), station(&mut Rng::new(seed)));
            let (sta_c, ap_c) = (sta.commit_frame().2, ap.commit_frame().2);
            let mut liar = station(&mut Rng::new(rng.next()));
            liar.on_frame(1, 0, &flip(&mut rng, &ap_c));
            assert!(liar.pmk().is_none(), "a lied commit");
            let SaeStep::Send { body: ap_cf, .. } = ap.on_frame(1, 0, &sta_c) else { panic!() };
            for s in [&mut sta, &mut twin] {
                assert!(matches!(s.on_frame(1, 0, &ap_c), SaeStep::Send { .. }));
            }
            sta.on_frame(2, 0, &flip(&mut rng, &ap_cf));
            assert!(sta.pmk().is_none() && sta.state() != SaeState::Accepted, "a lied confirm");
            twin.on_frame(2, 0, &ap_cf);
            assert_eq!(twin.state(), SaeState::Accepted, "the same exchange, honest");
        }
    }
}

#[test]
fn confirms_read_as_built_and_a_short_one_is_refused() {
    use nonos_wifi_core::sae::frame::{confirm_body, parse_confirm};
    super::rng::each(|rng| {
        let (sc, cn) = (rng.next() as u16, rng.bytes(32).try_into().unwrap());
        let body = [confirm_body(sc, &cn), rng.span(0, 8)].concat();
        assert_eq!(parse_confirm(&body), Some((sc, cn)));
        assert_eq!(parse_confirm(&body[..rng.below(34)]), None);
    });
}
