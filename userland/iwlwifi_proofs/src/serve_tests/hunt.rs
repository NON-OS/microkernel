// NONOS Operating System (AGPL-3.0-or-later)
//! The hunt stops the sweep once the network is heard, and an abort
//! completed before its reply ends the sweep.


use crate::gen3::join::hunt::hunt;
use crate::gen3::sweep::Sweep;
use crate::gen3_model::{reply, Seen};
use crate::join_rig::{beacon, rig, Security, Station, Switches, RSNE_PSK};
use super::scripted::{completion, heard, other_network};

#[test]
fn the_hunt_stops_the_sweep_once_the_network_is_heard() {
    let r = rig(Security::Wpa2, Switches::default());
    let target = beacon(&[&RSNE_PSK]);
    let t = target.clone();
    r.model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| match (c.group, c.cmd) {
        // The scan request: another network on 1, then the target on 6.
        (0x1, 0x0D) => vec![reply(c, vec![]), heard(&other_network(), 1), heard(&t, 6)],
        // The abort: the scan completes as aborted.
        (0x1, 0x0E) => vec![reply(c, vec![]), completion(2)],
        _ => vec![reply(c, vec![])],
    }));
    let mut st = Station::new();
    let mut seen = 0;
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        let mut t = 0u64;
        let got = hunt(fw.dev, fw.clock, &mut sweep, &[1, 6, 11], b"home", &mut || { t += 1; t }, &mut |_| seen += 1);
        assert_eq!(got, Ok((target.clone(), 6)));
        assert!(!sweep.running(), "the radio is the join's");
        assert_eq!(sweep.completed, 1);
    });
    assert_eq!(seen, 2, "every beacon also went to the scan list");
    let cmds: Vec<(u8, u8)> = r.commands();
    assert_eq!(cmds, vec![(0x1, 0x0D), (0x1, 0x0E)], "scan, then abort");
    let abort = r.command_payloads(0x1, 0x0E);
    assert_eq!(abort[0], vec![0u8; 8], "the abort names uid 0");
}

#[test]
fn an_abort_completed_before_its_reply_ends_the_sweep() {
    let r = rig(Security::Wpa2, Switches::default());
    let target = beacon(&[&RSNE_PSK]);
    let t = target.clone();
    r.model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| match (c.group, c.cmd) {
        (0x1, 0x0D) => vec![reply(c, vec![]), heard(&t, 11)],
        (0x1, 0x0E) => vec![completion(2), reply(c, vec![])],
        _ => vec![reply(c, vec![])],
    }));
    let mut st = Station::new();
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        let got = hunt(fw.dev, fw.clock, &mut sweep, &[11], b"home", &mut || 0, &mut |_| {});
        assert_eq!(got, Ok((target.clone(), 11)), "with the channel it was heard on");
        assert!(!sweep.running());
        assert_eq!(sweep.completed, 1);
    });
}
