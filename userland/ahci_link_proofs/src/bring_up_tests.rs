// NONOS Operating System (AGPL-3.0-or-later)
//! How the link bring-up reads PxSSTS after COMRESET, and the COMRESET
//! itself on a port register file in host memory, where no PHY ever answers.

use crate::constants::regs::{PORT_SCTL, PORT_SSTS, SCTL_DET_MASK};
use crate::engine::link::established::{link_state, LinkState};
use crate::engine::link::link_up;
use crate::error::AhciError;
use crate::regs::Regs;

#[test]
fn det_reads_as_up_negotiating_or_quiet() {
    assert_eq!(link_state(0x113), LinkState::Up);
    assert_eq!(link_state(0x623), LinkState::Up, "Gen2, slumber");
    assert_eq!(link_state(0x001), LinkState::Negotiating);
    for ssts in [0x000u32, 0x004, 0x002, 0x00f, 0x100] {
        assert_eq!(link_state(ssts), LinkState::Quiet, "{ssts:#x}");
    }
}

#[test]
fn an_empty_port_is_no_disk_soon_after_comreset() {
    let mut file = [0u32; 0x80 / 4];
    file[PORT_SCTL as usize / 4] = 0x300;
    let base = file.as_mut_ptr();
    let t = std::time::Instant::now();
    let r = link_up(Regs::new(base as u64), 0, 10_000);
    let took = t.elapsed();
    assert_eq!(r, Err(AhciError::NoDisk));
    // Quiet for LINK_EMPTY_MS, well inside LINK_TIMEOUT_MS.
    assert!(took.as_millis() >= 200 && took.as_millis() < 1_500, "{took:?}");
    // SAFETY: `base` points at `file`, which outlives the reads.
    let sctl = unsafe { core::ptr::read_volatile(base.add(PORT_SCTL as usize / 4)) };
    assert_eq!(sctl & SCTL_DET_MASK, 0, "PHY left in COMRESET");
    assert_eq!(sctl & !SCTL_DET_MASK, 0x300, "other PxSCTL fields changed");
    let ssts = unsafe { core::ptr::read_volatile(base.add(PORT_SSTS as usize / 4)) };
    assert_eq!(ssts, 0);
}

#[test]
fn a_device_up_and_idle_comes_up_at_once() {
    let mut file = [0u32; 0x80 / 4];
    file[PORT_SSTS as usize / 4] = 0x123;
    file[0x20 / 4] = 0x50;
    let base = file.as_mut_ptr();
    assert_eq!(link_up(Regs::new(base as u64), 0, 10_000), Ok(()));
}

#[test]
fn a_device_negotiating_forever_times_out() {
    let mut file = [0u32; 0x80 / 4];
    file[PORT_SSTS as usize / 4] = 0x001;
    let base = file.as_mut_ptr();
    assert_eq!(link_up(Regs::new(base as u64), 0, 10_000), Err(AhciError::Timeout));
}
