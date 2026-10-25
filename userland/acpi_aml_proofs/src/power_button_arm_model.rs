// NONOS Operating System (AGPL-3.0-or-later)
//! A model of QEMU's PM1 (hw/acpi/core.c) for the arming rule: a press
//! latches PWRBTN_STS only while PWRBTN_EN is set, and the SCI is raised
//! while any status bit is set beside its enable. Each check starts from the
//! worst a firmware can leave: the SCI line unmasked.

use crate::arch::x86_64::acpi::hw::power_button::PM1_STS_PWRBTN;
use crate::arch::x86_64::acpi::hw::power_button_arm::{ArmStep, PM1_EN_PWRBTN};

/// QEMU's ACPI_BITMASK_PM1_COMMON_ENABLED: timer, global lock, button, RTC.
pub(crate) const SCI_SOURCES: u16 = 0x0001 | 0x0020 | 0x0100 | 0x0400;

struct Machine {
    sts: u16,
    en: u16,
    masked: bool,
    unhandled_sci: bool,
}

impl Machine {
    fn press(&mut self) {
        if self.en & PM1_EN_PWRBTN != 0 {
            self.sts |= PM1_STS_PWRBTN;
        }
        self.sci();
    }
    fn sci(&mut self) {
        if self.sts & self.en & SCI_SOURCES != 0 && !self.masked {
            self.unhandled_sci = true;
        }
    }
    fn step(&mut self, s: ArmStep) {
        match s {
            ArmStep::MaskSci => self.masked = true,
            ArmStep::Enable(v) => self.en = v,
        }
        self.sci();
    }
}

/// A press at every point of arming: no SCI is ever raised unmasked, and once
/// armed a press is latched for the poll to read.
pub(crate) fn check(steps: &[ArmStep], en: u16) -> Result<(), &'static str> {
    for at in 0..=steps.len() {
        let mut m = Machine { sts: 0, en, masked: false, unhandled_sci: false };
        steps[..at].iter().for_each(|&s| m.step(s));
        m.press();
        steps[at..].iter().for_each(|&s| m.step(s));
        if m.unhandled_sci {
            return Err("an SCI was raised with its line unmasked");
        }
    }
    let mut m = Machine { sts: 0, en, masked: false, unhandled_sci: false };
    steps.iter().for_each(|&s| m.step(s));
    m.press();
    if m.sts & PM1_STS_PWRBTN == 0 {
        return Err("a press after arming was not latched");
    }
    Ok(())
}
