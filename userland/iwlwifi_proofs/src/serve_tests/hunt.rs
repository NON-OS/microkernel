// NONOS Operating System (AGPL-3.0-or-later)
//! The hunt hears the whole sweep, every access point of the network a
//! candidate, and an abort completed before its reply ends a background
//! sweep.

use nonos_wifi_core::bss::Candidates;
use nonos_wifi_core::rsn::JoinPolicy;

use super::scripted::{completion, heard, other_network};
use crate::gen3::join::hunt::hunt;
use crate::gen3::sweep::{Sweep, Tick};
use crate::gen3_model::{reply, Seen};
use crate::join_rig::{beacon, rig, Security, Station, Switches, RSNE_PSK};

#[test]
fn the_hunt_hears_the_whole_sweep() {
    let r = rig(Security::Wpa2, Switches::default());
    let target = beacon(&[&RSNE_PSK]);
    let t = target.clone();
    r.model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| match (c.group, c.cmd) {
        // The scan request: another network on 1, the target on 6, the end.
        (0x1, 0x0D) => vec![reply(c, vec![]), heard(&other_network(), 1), heard(&t, 6), completion(c, 1)],
        _ => vec![reply(c, vec![])],
    }));
    let mut st = Station::new();
    let (mut seen, mut found) = (0, Candidates::new(b"home", JoinPolicy::ANY));
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        let mut t = 0u64;
        let got = hunt(fw.dev, fw.clock, &mut sweep, &[1, 6, 11], &mut found, &mut || { t += 1; t }, &mut |_| seen += 1);
        assert_eq!(got, Ok(()));
        assert!(!sweep.running(), "the radio is the join's");
        assert_eq!(sweep.completed, 1);
    });
    assert_eq!(seen, 2, "every beacon also went to the scan list");
    assert_eq!(r.commands(), vec![(0x1, 0x0D)], "one sweep, run to its end");
    let best = found.take_best().expect("the network");
    assert_eq!((best.frame(), best.channel()), (&target[..], 6));
    assert!(found.take_best().is_none(), "the other network is no candidate");
}

#[test]
fn an_abort_completed_before_its_reply_ends_the_sweep() {
    let r = rig(Security::Wpa2, Switches::default());
    let target = beacon(&[&RSNE_PSK]);
    let t = target.clone();
    let first = std::cell::Cell::new(true);
    r.model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| match (c.group, c.cmd) {
        (0x1, 0x0D) if first.replace(false) => vec![reply(c, vec![])],
        (0x1, 0x0D) => vec![reply(c, vec![]), heard(&t, 11), completion(c, 1)],
        (0x1, 0x0E) => vec![completion(c, 2), reply(c, vec![])],
        _ => vec![reply(c, vec![])],
    }));
    let mut st = Station::new();
    let mut found = Candidates::new(b"home", JoinPolicy::ANY);
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        assert_eq!(sweep.start(fw.dev, fw.clock, &[11], 0, &mut |_| {}), Ok(Tick::Running));
        let got = hunt(fw.dev, fw.clock, &mut sweep, &[11], &mut found, &mut || 0, &mut |_| {});
        assert_eq!(got, Ok(()));
        assert!(!sweep.running());
    });
    assert_eq!(r.commands(), vec![(0x1, 0x0D), (0x1, 0x0E), (0x1, 0x0D)], "background, abort, the hunt's");
    assert_eq!(found.take_best().map(|c| c.channel()), Some(11), "where it was heard");
}
