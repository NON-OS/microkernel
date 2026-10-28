// NONOS Operating System (AGPL-3.0-or-later)
//! A WPA3-Personal access point in the modelled chip's air: SAE with
//! hash-to-element, management frame protection required, then the
//! four-way handshake under the PMK that SAE made. Its SAE half is the
//! shared SAE code's own `SaeStation` (the exchange is symmetric), and its
//! handshake messages the simulator's (`ap_sim`). Heard only on its channel,
//! beaconing every 100 ms of the chip's clock.

mod ap;
mod commit;
mod keys;

pub use ap::SaeAp;
use nonos_wifi_core::regdom::Band;
use std::collections::VecDeque;

pub const SSID: &[u8] = b"WPA3 home";
pub const PASS: &[u8] = b"correct horse battery";
/// RSNE: CCMP-128 group and pairwise, AKM SAE (00-0F-AC:8), MFPR and MFPC.
pub const RSNE_SAE: [u8; 22] =
    [0x30, 0x14, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f, 0xac, 8, 0xc0, 0];
/// RSNXE: SAE hash-to-element.
pub const RSNXE_H2E: [u8; 3] = [244, 1, 0x20];

pub struct SaeAir {
    pub ap: SaeAp,
    pub now: u64,
    pub tuned: Option<(Band, u8)>,
    beacon_at: u64,
    pub inbox: VecDeque<Vec<u8>>,
}

impl SaeAir {
    pub fn new(channel: u8) -> Self {
        let ap = SaeAp::new(channel);
        SaeAir { ap, now: 0, tuned: None, beacon_at: 0, inbox: VecDeque::new() }
    }

    fn on(&self) -> bool {
        self.tuned == Some((Band::of_legacy(self.ap.channel), self.ap.channel))
    }

    pub fn send(&mut self, mpdu: &[u8]) {
        let to = mpdu.get(4..10).unwrap_or(&[]);
        if self.on() && (to == crate::ap_sim::AA || to == [0xff; 6]) {
            let replies = self.ap.on_frame(mpdu);
            self.inbox.extend(replies);
        }
    }

    pub fn recv(&mut self) -> Option<Vec<u8>> {
        if let Some(f) = self.inbox.pop_front() {
            return Some(f);
        }
        if self.on() && self.now >= self.beacon_at {
            self.beacon_at = self.now + 100;
            return Some(self.ap.beacon(8, [0xff; 6]));
        }
        None
    }
}
