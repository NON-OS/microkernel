// NONOS Operating System (AGPL-3.0-or-later)

//! A WPA3 access point on the modelled air: the RTL8852 proofs' SAE access
//! point (`sae_air`, WPA3-Personal with PMF required) or its transition-mode
//! twin (`sae_mixed`). As the WPA2 air does, it is heard only on its
//! channel, beacons every 100 ms of the clock and once on a retune, and
//! answers what the chip sends to it or to everyone while there.

use crate::ap_sim::AA;

/// What the model needs of either access point.
pub trait Wpa3Ap {
    fn channel(&self) -> u8;
    fn beacon(&self) -> Vec<u8>;
    fn on_frame(&mut self, f: &[u8]) -> Vec<Vec<u8>>;
    /// The pairwise key once message 4 came; the station's data, opened.
    fn tk(&self) -> Option<[u8; 16]>;
    fn got(&self) -> &[Vec<u8>];
}

macro_rules! sae_ap {
    ($t:ty) => {
        impl Wpa3Ap for $t {
            fn channel(&self) -> u8 {
                self.channel
            }
            fn beacon(&self) -> Vec<u8> {
                <$t>::beacon(self, 8, [0xff; 6])
            }
            fn on_frame(&mut self, f: &[u8]) -> Vec<Vec<u8>> {
                <$t>::on_frame(self, f)
            }
            fn tk(&self) -> Option<[u8; 16]> {
                self.tk
            }
            fn got(&self) -> &[Vec<u8>] {
                &self.got
            }
        }
    };
}

sae_ap!(crate::sae_air::SaeAp);
sae_ap!(crate::sae_mixed::SaeAp);

pub struct Wpa3 {
    pub ap: Box<dyn Wpa3Ap>,
    beacon_at: u64,
    last: Option<u8>,
}

impl Wpa3 {
    pub fn new(ap: Box<dyn Wpa3Ap>) -> Wpa3 {
        Wpa3 { ap, beacon_at: 0, last: None }
    }

    pub fn on_frame(&mut self, f: &[u8], tuned: Option<u8>) -> Vec<Vec<u8>> {
        let to = f.get(4..10).unwrap_or(&[]);
        let here = tuned == Some(self.ap.channel());
        if !here || (to != AA && to != [0xff; 6]) || f.len() < 24 {
            return Vec::new();
        }
        self.ap.on_frame(f)
    }

    pub fn tick(&mut self, now: u64, tuned: Option<u8>) -> Vec<Vec<u8>> {
        let retuned = tuned != self.last;
        self.last = tuned;
        if tuned != Some(self.ap.channel()) || !(retuned || now >= self.beacon_at) {
            return Vec::new();
        }
        self.beacon_at = now + 100;
        vec![self.ap.beacon()]
    }
}
