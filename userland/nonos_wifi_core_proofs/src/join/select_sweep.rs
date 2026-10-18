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

//! Over seeded RSN elements and every join policy: a network saved as WPA3
//! is never settled as PSK, and a transition network (SAE with management
//! frame protection) is never silently taken as PSK. The selector is pure,
//! so the sweep drives it directly, the same each run.

use crate::rsn_tests::body;
use crate::sweep::rng::{each, Rng};
use nonos_wifi_core::rsn::{parse_rsne, select, JoinPolicy, Pmf, SelectError};
use nonos_wifi_core::wpa::akm::Akm;

/// Cipher suite bytes: TKIP(2), CCMP-128(4), GCMP-256(9), and a junk one.
const CIPHERS: [u8; 4] = [2, 4, 9, 13];
/// AKM bytes: 802.1X(1), PSK(2), PSK-SHA256(6), SAE(8), FT-SAE(9).
const AKMS: [u8; 5] = [1, 2, 6, 8, 9];

fn rsne(rng: &mut Rng) -> Vec<u8> {
    let group = rng.pick(&CIPHERS);
    let np = 1 + rng.below(2);
    let na = 1 + rng.below(3);
    let pairwise: Vec<u8> = (0..np).map(|_| rng.pick(&CIPHERS)).collect();
    let akms: Vec<u8> = (0..na).map(|_| rng.pick(&AKMS)).collect();
    let caps = rng.pick(&[0u16, 0x0080, 0x00C0, 0x0040]);
    body(group, &pairwise, &akms, caps)
}

fn psk(akm: Akm) -> bool {
    matches!(akm, Akm::Psk | Akm::PskSha256)
}

#[test]
fn wpa3_saved_is_never_joined_with_psk() {
    each(|rng| {
        let Some(r) = parse_rsne(&rsne(rng)) else { return };
        let h2e = rng.byte() & 1 == 1;
        // Saved as WPA3: SAE or a refusal, never a PSK AKM.
        match select(&r, h2e, JoinPolicy::WPA3_ONLY) {
            Ok(s) => {
                assert_eq!(s.akm, Akm::Sae, "WPA3-only settled on a non-SAE AKM");
                // A saved-WPA3 join demands PMF: it never connects without it.
                assert_ne!(s.pmf, Pmf::Off, "WPA3-only connected without protection");
            }
            Err(e) => assert!(matches!(
                e,
                SelectError::Downgrade
                    | SelectError::UnsupportedAkm
                    | SelectError::UnsupportedCipher
                    | SelectError::BadProtection
            )),
        }
        // A transition network offers SAE with protection; ANY takes SAE, so
        // stripping nothing lets an attacker force PSK on it.
        if r.akm_sae && r.mfpc() {
            if let Ok(s) = select(&r, h2e, JoinPolicy::ANY) {
                assert!(!psk(s.akm), "a protected-SAE network was taken as PSK");
            }
        }
    });
}
