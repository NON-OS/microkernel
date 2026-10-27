// NONOS Operating System (AGPL-3.0-or-later)

//! The chip's register file: plain storage, except the ownership register
//! (`MT_CONN_ON_LPCTL`), the firmware's ready bits (`MT_CONN_ON_MISC`), the
//! L1 window (`l1`), and the ring registers, whose `cpu_idx` writes make
//! the WFDMA move (`rings`). Every access also lets time pass on the air.

use std::collections::{HashMap, VecDeque};

use super::air::Air;
use super::data::TxFrame;
use super::faults::Faults;
use super::mcu::Mcu;
use super::wpa3::Wpa3;

pub const LPCTL: u32 = 0xe0010;
pub const MISC: u32 = 0xe00f0;
pub const REMAP: u32 = 0xfe24c;

pub struct Chip {
    pub regs: HashMap<u32, u32>,
    pub l1: HashMap<u32, u32>,
    /// `MT_HW_CHIPID`, `MT_HW_BOUND`, `MT_HW_REV`.
    pub id: (u32, u32, u32),
    pub own_sync: bool,
    pub fw_own: bool,
    pub faults: Faults,
    pub mcu: Mcu,
    pub air: Option<Air>,
    pub wpa3: Option<Wpa3>,
    /// The access point decodes no CCK (`phy`); frames it did not hear.
    pub g_only: bool,
    pub unheard: u32,
    pub pending: Vec<VecDeque<Vec<u8>>>,
    pub sent: Vec<TxFrame>,
    /// Every register write, by BAR offset, oldest first.
    pub writes: Vec<(u32, u32)>,
}

impl Chip {
    pub fn new(chipid: u32) -> Chip {
        Chip {
            regs: HashMap::new(),
            l1: HashMap::new(),
            id: (chipid, 0, 0x10),
            own_sync: false,
            fw_own: true,
            faults: Faults::default(),
            mcu: Mcu::default(),
            air: None,
            wpa3: None,
            g_only: false,
            unheard: 0,
            pending: (0..8).map(|_| VecDeque::new()).collect(),
            sent: Vec::new(),
            writes: Vec::new(),
        }
    }

    pub fn reg(&self, off: u32) -> u32 {
        self.regs.get(&off).copied().unwrap_or(0)
    }
}
