// NONOS Operating System (AGPL-3.0-or-later)
//! The chip as it comes out of reset on the modelled board.

use super::cpu::Fwdl;
use super::efuse_img;
use super::faults::Faults;
use super::state::State;
use std::collections::{HashMap, VecDeque};

impl State {
    pub fn new() -> Self {
        let mut s = State {
            regs: HashMap::new(),
            xtal: [0; 256],
            efuse: efuse_img::image(),
            dav: [0xff; 96],
            rf: [[0; 256]; 2],
            cfg: HashMap::from([(0x82, 0x2), (0x719, 0x8)]), // Gen2 link, L1 on
            lte: HashMap::new(),
            fwdl: Fwdl::Off,
            fw_hdr: Vec::new(),
            fw_got: Vec::new(),
            fw_tries: 0,
            h2c: Vec::new(),
            mailbox: Vec::new(),
            // Two TX and two RX streams, both antennas (`rtw89_mac_setup_phycap`).
            phy_cap: [0x2_0000, 2, 0, 0],
            powered: false,
            log: Vec::new(),
            bus: Vec::new(),
            sent: Vec::new(),
            rxq: VecDeque::new(),
            rpq: VecDeque::new(),
            rpp: Vec::new(),
            air: None,
            sae: None,
            cam: None,
            faults: Faults::default(),
            violations: Vec::new(),
        };
        s.regs.insert(0x8, 0x0020_0000); // efuse autoloaded (R_AX_SYS_WL_EFUSE_CTRL)
        s.regs.insert(0xf0, 0x1000); // R_AX_SYS_CFG1: cut CBV
        s.regs.insert(0xc348, 0xffff); // R_AX_CTN_TXEN: every queue on
        s
    }
}
