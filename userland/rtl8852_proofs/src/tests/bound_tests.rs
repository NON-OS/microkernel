// NONOS Operating System (AGPL-3.0-or-later)
//! Every wait ends, after rtw89's own timeout and not much later: a chip
//! that never answers costs the time Linux would give it, then the step
//! fails. Measured on the proofs' clock, which only waits move.

use crate::chan::Chan;
use crate::clock::peek_ms;
use crate::model::{rig, State};

fn took(f: impl FnOnce()) -> u64 {
    let t = peek_ms();
    f();
    peek_ms() - t
}

fn bring_took(setup: impl FnOnce(&mut State)) -> u64 {
    took(|| assert!(rig::bring(setup).1.is_err()))
}

#[test]
fn the_mailbox_waits_one_second_for_an_answer() {
    let (card, mut chip) = rig::ready();
    card.0.borrow_mut().faults.mute = true;
    let ms = took(|| assert!(chip.tune_to(Chan::new(false, 6).unwrap()).is_err()));
    assert!((1000..1010).contains(&ms), "{ms} ms");
}

#[test]
fn a_command_queue_that_never_moves_is_given_half_a_second() {
    let (card, mut chip) = rig::ready();
    card.0.borrow_mut().faults.h2c_stall = true;
    let sma = chip.vif.sma;
    // The ring takes 23 posts (32 less the 8 kept free for MULTITAG, less one).
    for _ in 0..23 {
        assert!(chip.cam_update(sma).is_ok());
    }
    let ms = took(|| assert!(chip.cam_update(sma).is_err()));
    assert!((500..510).contains(&ms), "{ms} ms");
}

#[test]
fn a_firmware_that_never_reports_ready_costs_five_status_waits() {
    let ms = bring_took(|s| s.faults.fw_status = Some(3));
    assert!((5 * 400..5 * 400 + 100).contains(&ms), "{ms} ms");
}

#[test]
fn a_dead_efuse_costs_one_cell_wait() {
    let ms = bring_took(|s| s.faults.efuse_dead = true);
    assert!((1000..1050).contains(&ms), "{ms} ms");
}

#[test]
fn power_that_never_comes_costs_two_tries_of_twenty_ms() {
    let ms = bring_took(|s| s.faults.stuck.push((0x4, 0x2_0000, false)));
    assert!((40..50).contains(&ms), "{ms} ms");
}
