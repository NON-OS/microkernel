// NONOS Operating System (AGPL-3.0-or-later)
//! The air between the modelled chip and a simulated AP: the Wi-Fi core
//! proofs' WPA2 air (`soft::rig::SimAir`) or this crate's WPA3 one
//! (`sae_air`). The channel the air hears is the one RF path A is
//! programmed to (RR_CFGCH), never the one the driver meant; the station
//! address the APs talk to is CAM entry 0's; and a received frame passes
//! the filter only when addressed to that address or to a group.

use super::rf;
use super::state::State;
use crate::soft::ap::SimAp;
use nonos_wifi_core::regdom::Band;
use nonos_wifi_core::softmac::Radio;

/// Put `ap` in the air around the chip.
pub fn attach(s: &mut State, ap: SimAp) {
    s.air = Some(crate::soft::air_new::air(ap));
}

fn sync(s: &mut State) {
    let tuned = rf::tuned(s).map(|(five, ch)| (if five { Band::Ghz5 } else { Band::Ghz2 }, ch));
    let (sma, now) = (s.cam.map(|c| c.0), crate::clock::peek_ms());
    if let Some(air) = s.air.as_mut() {
        air.clock.set(now);
        air.tuned = tuned;
        if let Some(a) = sma {
            air.addr.0 = a;
        }
    }
    if let Some(sae) = s.sae.as_mut() {
        (sae.now, sae.tuned) = (now, tuned);
    }
}

pub fn transmit(s: &mut State, mpdu: &[u8]) {
    sync(s);
    if let Some(air) = s.air.as_mut() {
        air.send(mpdu);
    }
    if let Some(sae) = s.sae.as_mut() {
        sae.send(mpdu);
    }
}

fn heard(s: &mut State, out: &mut [u8]) -> Option<Vec<u8>> {
    if let Some(air) = s.air.as_mut() {
        return air.recv(out).map(|rx| out[..rx.len].to_vec());
    }
    s.sae.as_mut()?.recv()
}

/// Take what the air carries now into the receive queue, through the filter.
pub fn listen(s: &mut State) {
    let Some(sma) = s.cam.map(|c| c.0) else { return };
    sync(s);
    let mut out = vec![0u8; 4096];
    for _ in 0..8 {
        let Some(f) = heard(s, &mut out) else { break };
        let a1 = f.get(4..10).unwrap_or(&[]);
        if a1 == sma || a1.first().is_some_and(|b| b & 1 != 0) {
            if let Some((w3, f)) = super::sec_air::open(s, &f) {
                s.rxq.push_back(super::rxbuf::wifi_with(&f, w3));
            }
        }
    }
}
