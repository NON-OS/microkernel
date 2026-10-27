// NONOS Operating System (AGPL-3.0-or-later)
//! Everything the modelled chip holds.

use super::bus::Mem;
use super::cpu::{Fwdl, H2c};
use super::faults::Faults;
use super::txq::Sent;
use crate::sae_air::SaeAir;
use crate::soft::rig::SimAir;
use std::collections::{HashMap, VecDeque};

pub struct State {
    /// The register file by 32-bit word address (BB at 0x10000 on).
    pub regs: HashMap<u32, u32>,
    pub xtal: [u8; 256],
    /// The efuse's physical cells (main map, then phycap at 0x580).
    pub efuse: Vec<u8>,
    /// The DAV efuse cells, behind the XTAL SI bus.
    pub dav: [u8; 96],
    /// RF registers per path, 20 bits each, as SWSI reaches them.
    pub rf: [[u32; 256]; 2],
    /// The PCIe configuration space, by byte, as DBI reaches it.
    pub cfg: HashMap<u16, u8>,
    /// The LTE coexistence block's indirect registers.
    pub lte: HashMap<u32, u32>,
    pub fwdl: Fwdl,
    /// The last download: the header as sent, then every section byte.
    pub fw_hdr: Vec<u8>,
    pub fw_got: Vec<u8>,
    /// How many times the CPU was started for a download.
    pub fw_tries: u32,
    /// Every H2C command the running firmware took, in order.
    pub h2c: Vec<H2c>,
    /// Every register-mailbox message the firmware took: its four words.
    pub mailbox: Vec<[u32; 4]>,
    /// The PHY_CAP answer's four words.
    pub phy_cap: [u32; 4],
    /// The MAC powered (`APFN_ONMAC` seen, `APFM_OFFMAC` not since).
    pub powered: bool,
    /// Every word write: (address, value after).
    pub log: Vec<(u32, u32)>,
    pub bus: Vec<Mem>,
    /// Frames the chip put on the air.
    pub sent: Vec<Sent>,
    /// Buffers waiting for room in the receive and release-report rings.
    pub rxq: VecDeque<Vec<u8>>,
    pub rpq: VecDeque<Vec<u8>>,
    /// Release reports not yet put in a buffer.
    pub rpp: Vec<u32>,
    pub air: Option<SimAir>,
    pub sae: Option<SaeAir>,
    /// The station address and BSSID of address CAM entry 0.
    pub cam: Option<([u8; 6], [u8; 6])>,
    pub faults: Faults,
    pub violations: Vec<String>,
}

impl State {
    pub fn word(&self, a: u32) -> u32 {
        self.regs.get(&a).copied().unwrap_or(0)
    }
}
