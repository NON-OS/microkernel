// NONOS Operating System (AGPL-3.0-or-later)
//! A firmware error ends the sweep, and a sweep needs channels and an answer.


use crate::gen3::dev::WaitError;
use crate::gen3::regs::*;
use crate::gen3::sweep::{ScanError, Sweep, Tick};
use crate::gen3_model::Seen;
use super::rig::{booted, rig};

#[test]
fn a_firmware_error_ends_the_sweep() {
    let r = rig();
    booted(&r, |dev, c| {
        let mut sweep = Sweep::new();
        assert_eq!(sweep.start(dev, c, &[1], 0, &mut |_| {}), Ok(Tick::Running));
        r.model.s.borrow_mut().regs.insert(CSR_INT, INT_BIT_SW_ERR);
        assert_eq!(sweep.pump(dev, 1, &mut |_| {}), Tick::Failed);
        assert!(!sweep.running());
        assert_eq!(sweep.pump(dev, 2, &mut |_| {}), Tick::Failed, "the cause stays visible when idle");
    });
}

#[test]
fn a_sweep_needs_channels_and_an_answer() {
    let r = rig();
    booted(&r, |dev, c| {
        let mut sweep = Sweep::new();
        assert_eq!(sweep.start(dev, c, &[], 0, &mut |_| {}), Err(ScanError::NoChannels));
        assert_eq!(sweep.start(dev, c, &[1; 68], 0, &mut |_| {}), Err(ScanError::NoChannels), "over 67");
        r.model.s.borrow_mut().responder = Some(Box::new(|_: &Seen| vec![]));
        assert_eq!(sweep.start(dev, c, &[1], 0, &mut |_| {}), Err(ScanError::Request(WaitError::TimedOut)));
        assert!(!sweep.running());
    });
}
