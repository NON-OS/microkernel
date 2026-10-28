// NONOS Operating System (AGPL-3.0-or-later)
//! A modelled RTL8153 behind its vendor requests: a register file of
//! dwords per block, written lane by lane as the byte enables say, the PHY
//! behind its window, and the few bits the chip sets on its own (settings
//! loaded, PHY on, resets done, link list ready) set only after a few
//! looks, or never for the one a test makes stuck. Every access is logged.

mod mac;
mod phy;
mod plug;
mod vendor;

pub use plug::{plug, plug_with, Plugged};

use std::collections::HashMap;

pub const PLA: u16 = 0x0100;
pub const USB: u16 = 0x0000;

/// A readiness the chip never reaches, for the bound proofs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stuck {
    Nothing,
    Autoload,
    PhySettle,
    PhyOn,
    MacReset,
    LinkList,
    Aldps,
    PhyReset,
    Drain,
}

/// (block, dword address) read; (block, dword, enables, data) written;
/// a PHY register read or written through the window.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Access {
    Read(u16, u16),
    Write(u16, u16, u16, u32),
    PhyRead(u16),
    PhyWrite(u16, u16),
}

pub struct Chip {
    pub code: u16,
    pub dwords: HashMap<(u16, u16), u32>,
    pub phy: HashMap<u16, u16>,
    pub page: u16,
    pub log: Vec<Access>,
    /// The link word's low half: up, speed and duplex.
    pub link: u32,
    pub stuck: Stuck,
    /// Looks before each readiness comes, by name.
    pub left: HashMap<&'static str, u32>,
    /// Vendor transfers answered before every later one fails (EPROTO).
    pub fail_after: Option<usize>,
    /// Vendor transfers that fail (EPROTO) before the first is answered.
    pub flaky: usize,
    pub transfers: usize,
}

impl Chip {
    pub fn new(code: u16) -> Self {
        let mut phy = HashMap::new();
        phy.insert(0xa420, 5); // powered down
        phy.insert(0xa400, 0x1940); // BMCR: power-down, autoneg on
        let (dwords, log, left) = (HashMap::new(), Vec::new(), HashMap::new());
        let (link, stuck, fail_after, transfers) = (0, Stuck::Nothing, None, 0);
        let page = 0;
        Self { code, dwords, phy, page, log, link, stuck, left, fail_after, flaky: 0, transfers }
    }
}
