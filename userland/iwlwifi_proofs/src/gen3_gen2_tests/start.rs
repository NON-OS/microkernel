// NONOS Operating System (AGPL-3.0-or-later)

//! The 22000 family's start: a set persistence bit cleared unless
//! write-protected, and the AX200's BISR waits around each NIC init, in its
//! start and again in its boot.

use super::rig::{polls, rig, run, CC, QU_B, QU_B0};
use crate::gen3::region::Clock;
use crate::gen3::start::gen2::start_hw_gen2;
use crate::gen3::start::StartError;
use crate::gen3_model::{Mem, Model, Polls};

// The model's clock, keeping each wait asked of it.
struct Waits(Polls, Vec<u32>);

impl Clock for Waits {
    fn poll_for(&mut self, ms: u32, ready: &mut dyn FnMut() -> bool) -> bool {
        self.0.poll_for(ms, ready)
    }
    fn delay_us(&mut self, us: u32) {
        self.1.push(us);
        self.0.delay_us(us)
    }
    fn now_ms(&mut self) -> u64 {
        self.0.now_ms()
    }
}

#[test]
fn the_ax200s_boot_waits_out_its_bisr_around_both_nic_inits() {
    for (blob, rev, discrete, n) in [(CC, 0x340, true, 2), (QU_B, QU_B0, false, 0)] {
        let r = rig(blob, rev, discrete);
        let mut c = Waits(polls(), Vec::new());
        run(&r, &mut c, |_, _, _| ()).0.expect("alive");
        let count = |us| c.1.iter().filter(|&&d| d == us).count();
        assert_eq!((count(2000), count(200)), (n, n), "discrete: {discrete}");
    }
}

fn bare() -> Model {
    let m = Model::new(Mem::new(4096, 0x1000_0000), Mem::new(4096, 0x2000_0000));
    m.s.borrow_mut().gen2 = true;
    m
}

#[test]
fn a_persistence_bit_is_cleared_unless_write_protected() {
    let m = bare();
    m.s.borrow_mut().prph_vals.insert(0x0_3440, 1 << 12 | 1);
    assert_eq!(start_hw_gen2(&m, &mut polls(), false, false), Ok(()));
    assert!(m.s.borrow().prph.contains(&(0x0_3440, 1)), "HPM_DEBUG without the bit");
    let m = bare();
    m.s.borrow_mut().prph_vals.extend([(0x0_3440, 1 << 12), (0x0_4D00, 1 << 12)]);
    assert_eq!(start_hw_gen2(&m, &mut polls(), false, false), Err(StartError::Persistent));
    assert!(m.s.borrow().prph.is_empty(), "nothing written");
    let m = bare();
    m.s.borrow_mut().prph_vals.insert(0x0_3440, 0xA5A5_A5A2);
    assert_eq!(start_hw_gen2(&m, &mut polls(), false, false), Ok(()));
    assert!(m.s.borrow().prph.is_empty(), "a hardware error value is left alone");
}

#[test]
fn the_ax200_waits_out_its_bisr() {
    let time = |bisr| {
        let mut c = polls();
        start_hw_gen2(&bare(), &mut c, false, bisr).unwrap();
        c.delays_us
    };
    assert_eq!(time(true) - time(false), 2000 + 200);
}
