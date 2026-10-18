// NONOS Operating System (AGPL-3.0-or-later)
//! The HBA around the model port, for a test that opens the controller:
//! CAP, PI and GHC, the port's registers at PORT_BASE as port 0 and any
//! further ports after it (`hba_bus`, `hba_ports`). GHC.HR resets each port
//! as AHCI 1.3.1, 10.4.3 has it: once HR reads clear, PxCMD.ST, CR, FRE and
//! FR read 0 (CR stays on where it never clears, `CrStuck::Always`; all stay
//! on a port the reset spares, `hr_spares_port`) and nothing is issued.

use super::hba::Hba;
use crate::constants::regs::{CAP_S64A, GHC_AE, GHC_HR, HBA_CAP, HBA_GHC, HBA_PI, HBA_VS};

/// GHC reads that still show HR set after it is written.
const HR_READS: u32 = 2;

pub struct Global {
    pub cap: u32,
    pub pi: u32,
    pub ghc: u32,
    /// HR never clears: the HBA reset never ends.
    pub hr_stuck: bool,
    hr_left: Option<u32>,
    /// HBA resets that ended, HR read back clear.
    pub resets: u32,
    /// Ports 1 and up, behind port 0 (`hba_ports`).
    pub extra: Vec<Hba>,
}

impl Global {
    /// `n` ports (1 to 32), 64-bit addressing, in AHCI mode as the firmware
    /// left it; ports 1 and up each with a drive of its own.
    pub fn ports(n: u8) -> Self {
        let extra = (1..n).map(|_| Hba::new(0, Vec::new())).collect();
        let (cap, pi) = (CAP_S64A | u32::from(n - 1), ((1u64 << n) - 1) as u32);
        Self { cap, pi, ghc: GHC_AE, hr_stuck: false, hr_left: None, resets: 0, extra }
    }

    /// GHC as read: HR shows until the reset ends; true when this read sees
    /// it end.
    fn read_ghc(&mut self) -> (u32, bool) {
        match self.hr_left {
            Some(n) if n > 0 || self.hr_stuck => self.hr_left = Some(n.saturating_sub(1)),
            Some(_) => {
                (self.hr_left, self.ghc, self.resets) = (None, self.ghc & !GHC_HR, self.resets + 1);
                return (self.ghc, true);
            }
            None => {}
        }
        (self.ghc, false)
    }
}

impl Hba {
    pub(super) fn read_global(&mut self, off: u32) -> u32 {
        let Some(g) = self.global.as_mut() else { return 0 };
        let (ghc, ended) = if off == HBA_GHC { g.read_ghc() } else { (g.ghc, false) };
        let v = [(HBA_CAP, g.cap), (HBA_PI, g.pi), (HBA_VS, 0x1_0300), (HBA_GHC, ghc)];
        let v = v.iter().find_map(|&(at, v)| (at == off).then_some(v)).unwrap_or(0);
        if ended {
            self.hr_reset_ports();
        }
        v
    }

    pub(super) fn write_global(&mut self, off: u32, v: u32) {
        let Some(g) = self.global.as_mut() else { return };
        match off {
            HBA_GHC if v & GHC_HR != 0 => (g.ghc, g.hr_left) = (GHC_HR, Some(HR_READS)),
            HBA_GHC => g.ghc = v | g.ghc & GHC_HR,
            HBA_CAP => g.cap = v,
            HBA_PI => g.pi = v,
            _ => {}
        }
    }
}
