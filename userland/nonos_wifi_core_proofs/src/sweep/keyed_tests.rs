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
//! No lie to message 3 changes a keyed state: one octet changed, a key data
//! length past the frame, wrapped data not whole blocks, or an authentic
//! message whose key data lies installs no key.

use super::keyed_lies::lie;
use super::rng::{Rng, SEEDS};
use crate::ap_sim::*;
use crate::handshake::sim::{configured, psk, ptk_for};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::State;

const ROUNDS: usize = 150;

#[test]
fn no_lie_to_message_3_installs_a_key() {
    let (pmk, kd) = (psk(), [&AP_RSNE_MIXED[..], &gtk_kde(1, &GTK)].concat());
    let ptk = ptk_for(Akm::Psk, &pmk, &ANONCE);
    for seed in SEEDS {
        let mut rng = Rng::new(seed);
        for round in 0..ROUNDS {
            let mut sup = configured(Akm::Psk, pmk, &STA_RSNE_PSK, &AP_RSNE_MIXED);
            sup.step(&message1(Akm::Psk, 1, &ANONCE));
            let tk = sup.tk().to_vec();
            let m3 = message3(Akm::Psk, 2, &ANONCE, &ptk, &kd);
            let lie = lie(&mut rng, round % 4, &m3, &kd, &ptk);
            assert!(sup.step(&lie).reply.is_none(), "seed {seed:#x} round {round}");
            assert!(matches!(sup.state(), State::PtkDerived | State::Failed));
            assert!(sup.gtk().is_empty() && sup.igtk().is_none(), "no group key");
            assert_eq!(sup.tk(), &tk[..], "the pairwise key as it was");
        }
    }
}
