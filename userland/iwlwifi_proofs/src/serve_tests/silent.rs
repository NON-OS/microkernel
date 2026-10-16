// NONOS Operating System (AGPL-3.0-or-later)
//! A network not heard is not found, and a background sweep is stopped
//! before the hunt.


use crate::gen3::join::hunt::{hunt, HuntEnd};
use crate::gen3::sweep::{Sweep, Tick};
use crate::gen3_model::{reply, Seen};
use crate::join_rig::{rig, Security, Station, Switches};
use super::scripted::{completion, heard, other_network};

#[test]
fn a_network_not_heard_in_a_whole_sweep_is_not_found() {
    let r = rig(Security::Wpa2, Switches::default());
    r.model.s.borrow_mut().responder = Some(Box::new(|c: &Seen| match (c.group, c.cmd) {
        (0x1, 0x0D) => vec![reply(c, vec![]), heard(&other_network(), 1), completion(c, 1)],
        _ => vec![reply(c, vec![])],
    }));
    let mut st = Station::new();
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        let got = hunt(fw.dev, fw.clock, &mut sweep, &[1, 6], b"home", &mut || 0, &mut |_| {});
        assert_eq!(got, Err(HuntEnd::NotHeard(1)));
        assert!(!sweep.running());
    });
    assert_eq!(r.commands(), vec![(0x1, 0x0D)], "nothing to abort");
}

#[test]
fn a_background_sweep_is_stopped_before_the_hunt_and_a_silent_one_is_given_up() {
    let r = rig(Security::Wpa2, Switches::default());
    let first = std::cell::Cell::new(true);
    r.model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| match (c.group, c.cmd) {
        (0x1, 0x0D) if first.replace(false) => vec![reply(c, vec![])],
        (0x1, 0x0D) => vec![reply(c, vec![]), completion(c, 1)],
        (0x1, 0x0E) => vec![reply(c, vec![]), completion(c, 2)],
        _ => vec![reply(c, vec![])],
    }));
    let mut st = Station::new();
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        assert_eq!(sweep.start(fw.dev, fw.clock, &[1], 0, &mut |_| {}), Ok(Tick::Running));
        let got = hunt(fw.dev, fw.clock, &mut sweep, &[1], b"home", &mut || 0, &mut |_| {});
        assert_eq!(got, Err(HuntEnd::NotHeard(0)));
    });
    assert_eq!(r.commands(), vec![(0x1, 0x0D), (0x1, 0x0E), (0x1, 0x0D)], "background, abort, the hunt's own");

    // A firmware that never completes the aborted sweep leaves the radio
    // busy: the hunt gives up rather than join under a running scan.
    let r = rig(Security::Wpa2, Switches::default());
    r.model.s.borrow_mut().responder = Some(Box::new(|c: &Seen| vec![reply(c, vec![])]));
    let mut st = Station::new();
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        assert_eq!(sweep.start(fw.dev, fw.clock, &[1], 0, &mut |_| {}), Ok(Tick::Running));
        let got = hunt(fw.dev, fw.clock, &mut sweep, &[1], b"home", &mut || 0, &mut |_| {});
        assert_eq!(got, Err(HuntEnd::Radio));
    });
}
