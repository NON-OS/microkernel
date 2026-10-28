// NONOS Operating System (AGPL-3.0-or-later)

//! Claim the chip, enable its memory space and map its register window
//! through the broker. A failure after the claim gives the claim back, so
//! the next attempt can claim it again.

use nonos_libc::{mk_device_claim, mk_device_release, mk_mmio_map, MmioMapOut};

use super::claim_note::{clear, note, STEP_CLAIM, STEP_MAP, STEP_MEMORY};
use super::master::command_rc;
use crate::discover::Found;
use crate::regs::Regs;

/// A claimed chip with its register window.
pub struct Claimed {
    pub regs: Regs,
    pub device_id: u64,
    pub epoch: u64,
}

pub fn run(dev: Found) -> Result<Claimed, &'static str> {
    let epoch = mk_device_claim(dev.device_id);
    if epoch <= 0 {
        note(STEP_CLAIM, epoch);
        return Err("rtl8852be: claim refused");
    }
    // Memory Space alone. Bus Master waits until the power cycle showed the
    // MAC off: a card whose last driver died may still have DMA armed.
    let rc = command_rc(dev.device_id, epoch as u64, false);
    if rc < 0 {
        note(STEP_MEMORY, rc);
        let _ = mk_device_release(dev.device_id);
        return Err("rtl8852be: memory space refused");
    }
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let r = mk_mmio_map(dev.device_id, epoch as u64, dev.bar_index, 0, 0, dev.bar_size, &mut out);
    if r < 0 {
        note(STEP_MAP, r);
        let _ = mk_device_release(dev.device_id);
        return Err("rtl8852be: register map refused");
    }
    clear();
    Ok(Claimed {
        // SAFETY: the broker mapped `length` bytes of the BAR at `user_va`
        // for this claim, which lasts until the driver releases it.
        regs: unsafe { Regs::new(out.user_va, out.length) },
        device_id: dev.device_id,
        epoch: epoch as u64,
    })
}
