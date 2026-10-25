// NONOS Operating System (AGPL-3.0-or-later)
//! Beacons, their elements and rates, the RSN and RSNX elements.

use nonos_wifi_core::dot11::ies::{ap_rates, beacon_tags, find_element, EID_RSN, EID_RSNX};
use nonos_wifi_core::dot11::parse::parse_beacon;
use nonos_wifi_core::rsn::parse_rsne;
use nonos_wifi_core::rsn::rsnxe::advertises_h2e;

use super::rng::within;
use crate::ap_sim::{AA, AP_RSNE_MIXED};
use crate::sweep::fuzz::Rng;

pub(super) const ROUNDS: u32 = 50_000;

fn beacon_seed() -> Vec<u8> {
    let mut f = vec![0x80, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    f.extend_from_slice(&[AA, AA].concat());
    f.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x64, 0, 0x11, 0x04, 0, 4]);
    f.extend_from_slice(b"home");
    f.extend_from_slice(&[1, 4, 0x82, 0x84, 0x8B, 0x96, 3, 1, 6]);
    f.extend_from_slice(&AP_RSNE_MIXED);
    f.extend_from_slice(&[244, 1, 0x20, 50, 2, 0x0C, 0x18]);
    f
}

#[test]
fn beacon_parsers_stay_inside_the_frame() {
    let (seed, mut r) = (beacon_seed(), Rng::seeded(0xBEAC0));
    for round in 0..ROUNDS {
        let f = if round % 4 == 0 { r.noise(300) } else { r.mutate(&seed) };
        if let Some(b) = parse_beacon(&f) {
            assert!(within(&f, b.ssid) && b.ssid.len() <= 32, "round {round}");
        }
        let Some(tags) = beacon_tags(&f) else { continue };
        assert!(within(&f, tags), "round {round}");
        for id in [EID_RSN, EID_RSNX, 0, 1, 3, 50] {
            if let Some(e) = find_element(tags, id) {
                assert!(
                    within(tags, e) && e[0] == id && e.len() == e[1] as usize + 2,
                    "round {round}"
                );
                let _ = (parse_rsne(&e[2..]), advertises_h2e(&e[2..]));
            }
        }
        let rates = ap_rates(tags);
        assert!(rates.supported().len() <= 8 && rates.len <= rates.rates.len(), "round {round}");
    }
}
