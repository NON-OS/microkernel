// NONOS Operating System (AGPL-3.0-or-later)

//! What a soft-MAC chip driver gives the shared station: a radio that tunes,
//! sends and receives whole 802.11 frames, and an environment with a clock
//! and a source of randomness. Everything above the frames (scan, join,
//! keys, the data path, the control and link protocols) is the core's, so a
//! chip driver is its registers and rings and nothing else.

use super::assoc::Association;
pub use super::env::Env;
pub use crate::key::{ChipKey, Crypto};
use crate::regdom::Band;
use crate::rsn::Ciphers;

/// One received frame: its length in the caller's buffer (FCS removed), the
/// signal it arrived at, and whether the chip decrypted it, leaving the CCMP
/// header and the MIC as received. Where the station holds the key it checks
/// that MIC itself; only a station that keyed the chip (`Crypto`) takes its
/// word. A chip that strips both writes the header back from the packet
/// number it reports, and eight octets where the MIC was.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RxInfo {
    pub len: usize,
    pub rssi_dbm: i8,
    pub decrypted: bool,
}

pub trait Radio {
    /// The station address the radio transmits from.
    fn mac(&self) -> [u8; 6];
    /// Transmit from, and receive for, `mac` from now on; false if the chip
    /// could not take it. The station gives each network and each scan that
    /// transmits its own address (`crate::address`).
    fn set_mac(&mut self, mac: [u8; 6]) -> bool;
    /// Which bands the radio has: 2.4 GHz, 5 GHz.
    fn bands(&self) -> (bool, bool);
    /// Retune to a 20 MHz channel; false if the chip refused it.
    fn tune(&mut self, band: Band, channel: u8) -> bool;
    /// Queue one complete MPDU without FCS; false if it could not be queued.
    fn send(&mut self, mpdu: &[u8]) -> bool;
    /// The next received frame, copied into `out`, or `None` for now.
    fn recv(&mut self, out: &mut [u8]) -> Option<RxInfo>;
    /// Point the receive filter at one BSS, or open it to every network.
    fn set_bss(&mut self, bssid: Option<[u8; 6]>);
    /// Periodic chip work (calibration, temperature compensation), called
    /// from every serve pass with the uptime.
    fn service(&mut self, _now_ms: u64) {}
    /// What association settled, once the AP accepts us (`None` when the
    /// station leaves or the join fails after it). A chip that runs rate
    /// adaptation or contention timing itself needs it; others ignore it.
    fn associated(&mut self, _assoc: Option<&Association>) {}
    /// The ciphers this radio runs beyond CCMP-128. A radio that does not
    /// say is held to CCMP-128, so a join with it keeps its old ciphers.
    fn ciphers(&self) -> Ciphers {
        Ciphers::CCMP_128_ONLY
    }
    /// Who runs CCMP-128: the station, unless the radio says the chip does.
    fn crypto(&self) -> Crypto {
        Crypto::Software
    }
    /// Put `key` into the chip's key table, keeping no copy; false if refused.
    fn set_key(&mut self, _key: &ChipKey<'_>) -> bool {
        false
    }
    /// Queue a frame for the chip to encrypt under the pairwise key it holds
    /// (Protected set, no MIC); false, and nothing sent, if it holds none.
    fn send_keyed(&mut self, _mpdu: &[u8]) -> bool {
        false
    }
    /// Chip-specific words the status op carries for a person diagnosing a
    /// bring-up on a machine with no serial console.
    fn diag(&self) -> [u32; 5] {
        [0; 5]
    }
}
