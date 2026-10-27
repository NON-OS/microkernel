// NONOS Operating System (AGPL-3.0-or-later)

//! The running chip, as the station drives it: its registers, rings and
//! firmware counters, the PHY and calibration state the bring-up left,
//! and what rtw89's `rtw89_vif` keeps for the one station interface.

use crate::chan::Chan;
use crate::dma::DmaMem;
use crate::h2c::cam_keys::Keys;
use crate::h2c::media::Net;
use crate::h2c::Fw;
use crate::phy::state::Phy;
use crate::regs::Mmio;
use crate::rfk::Rfk;
use crate::ring::Rings;

pub struct Chip<R: Mmio, M: DmaMem> {
    pub r: R,
    pub rings: Rings<M>,
    pub fw: Fw,
    pub phy: Phy,
    pub rfk: Rfk,
    pub chan: Option<Chan>,
    pub vif: Vif,
}

/// `struct rtw89_vif`'s part this driver uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vif {
    /// The address the station transmits from.
    pub sma: [u8; 6],
    /// The AP's, zero when none is chosen.
    pub bssid: [u8; 6],
    pub net: Net,
    pub aid: u16,
    /// The keys the address CAM entry points at.
    pub keys: Keys,
    pub beacon_int: u16,
    /// The AP's rates as learnt from its beacons (`tx::rate::learn`).
    pub rates: [u8; 16],
    pub n_rates: usize,
    pub scanning: bool,
    /// The EDCCA levels saved while scanning (`rtw89_phy_config_edcca`).
    pub edcca_bak: Option<[u32; 3]>,
}

impl Vif {
    pub const fn new(sma: [u8; 6]) -> Vif {
        Vif {
            sma,
            bssid: [0; 6],
            net: Net::NoLink,
            aid: 0,
            keys: Keys { valid: 0, keyid: [0; 7] },
            beacon_int: 0,
            rates: [0; 16],
            n_rates: 0,
            scanning: false,
            edcca_bak: None,
        }
    }

    pub fn rates(&self) -> &[u8] {
        &self.rates[..self.n_rates]
    }
}
