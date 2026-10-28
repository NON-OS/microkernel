// NONOS Operating System (AGPL-3.0-or-later)

//! The air around the modelled chip: one access point (the Wi-Fi core
//! proofs' `SimAp`), heard only while the chip's channel manager holds its
//! channel. It beacons every 100 ms of the clock and once on a retune, with
//! any data it has for the station; it answers what the chip sends it, or
//! to everyone, while on its channel.

use crate::soft::ap::SimAp;

pub struct Air {
    pub ap: SimAp,
    pub rssi: i8,
    beacon_at: u64,
    last: Option<u8>,
    /// The station's address, as its last frame to the AP gave it.
    pub sta: [u8; 6],
    /// Frames the AP was sent while the radio was elsewhere.
    pub missed: u32,
}

impl Air {
    pub fn new(ap: SimAp) -> Air {
        Air { ap, rssi: -50, beacon_at: 0, last: None, sta: [0; 6], missed: 0 }
    }

    fn here(&self, tuned: Option<u8>) -> bool {
        self.ap.on && tuned == Some(self.ap.channel)
    }

    pub fn on_frame(&mut self, f: &[u8], tuned: Option<u8>) -> Vec<Vec<u8>> {
        let to = f.get(4..10).unwrap_or(&[]);
        if to != self.ap.bssid && to != [0xff; 6] {
            return Vec::new();
        }
        if !self.here(tuned) {
            self.missed += 1;
            return Vec::new();
        }
        self.sta.copy_from_slice(&f[10..16]);
        self.ap.on_frame(f)
    }

    pub fn tick(&mut self, now: u64, tuned: Option<u8>) -> Vec<Vec<u8>> {
        let retuned = tuned != self.last;
        self.last = tuned;
        if !self.here(tuned) || !(retuned || now >= self.beacon_at) {
            return Vec::new();
        }
        self.beacon_at = now + 100;
        let mut out = vec![self.ap.beacon_out()];
        out.extend(self.ap.chat(self.sta));
        out
    }
}
