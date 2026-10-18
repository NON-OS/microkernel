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
//! RSN element bodies built sound (802.11-2020, 9.4.2.24): each field the
//! building stops after, and the reading it implies. Fields after the
//! version are optional, so a sound body may end after any of them.

use super::rng::Rng;
use super::rsne_shape::{defaults, suite, Field, Sound};
use nonos_wifi_core::rsn::suite::{selector, AKM_PSK, AKM_PSK_SHA256, AKM_SAE};
use nonos_wifi_core::rsn::suite::{CIPHER_CCMP, CIPHER_GCMP_256};

pub fn build(rng: &mut Rng) -> Sound {
    let want = defaults();
    let mut s = Sound { body: Vec::new(), want, fields: Vec::new() };
    s.put(Field::Version, &[1, 0]);
    let stop = rng.below(7);
    if stop >= 1 {
        let g = suite(rng, &[2, 4, 9]);
        s.want.group = selector(g);
        s.put(Field::Fixed, &g);
    }
    if stop >= 2 {
        s.want.pairwise_ccmp = false;
        for t in super::rsne_lists::list(&mut s, rng, &[1, 2, 4, 9], 4) {
            s.want.pairwise_ccmp |= t == CIPHER_CCMP;
            s.want.pairwise_gcmp256 |= t == CIPHER_GCMP_256;
        }
    }
    if stop >= 3 {
        for t in super::rsne_lists::list(&mut s, rng, &[1, 2, 6, 8], 4) {
            s.want.akm_psk |= t == AKM_PSK;
            s.want.akm_psk_sha256 |= t == AKM_PSK_SHA256;
            s.want.akm_sae |= t == AKM_SAE;
        }
    }
    super::rsne_lists::rest(&mut s, rng, stop);
    s
}
