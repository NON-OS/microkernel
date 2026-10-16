// NONOS Operating System (AGPL-3.0-or-later)
//! The hunt joins only where the world domain allows: a beacon heard on
//! channel 14 is not taken, and one heard near the channel it names is
//! joined on that channel.

use super::scripted::{completion, heard};
use crate::gen3::join::hunt::{hunt, HuntEnd};
use crate::gen3::sweep::Sweep;
use crate::gen3_model::{reply, Seen};
use crate::join_rig::{beacon, rig, Security, Station, Switches, RSNE_PSK};

// The rig's beacon names channel 6; it is heard on `on`. With `abort` the
// scan answers an abort, else it completes on its own.
fn hunt_heard_on(on: u8, abort: bool) -> Result<(Vec<u8>, u8), HuntEnd> {
    let r = rig(Security::Wpa2, Switches::default());
    let t = beacon(&[&RSNE_PSK]);
    r.model.s.borrow_mut().responder = Some(Box::new(move |c: &Seen| match (c.group, c.cmd) {
        (0x1, 0x0D) if abort => vec![reply(c, vec![]), heard(&t, on)],
        (0x1, 0x0D) => vec![reply(c, vec![]), heard(&t, on), completion(c, 1)],
        (0x1, 0x0E) => vec![reply(c, vec![]), completion(c, 2)],
        _ => vec![reply(c, vec![])],
    }));
    let mut st = Station::new();
    let mut got = Err(HuntEnd::Radio);
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        got = hunt(fw.dev, fw.clock, &mut sweep, &[1, 6, 14], b"home", &mut || 0, &mut |_| {});
        assert!(!sweep.running());
    });
    got
}

#[test]
fn a_beacon_heard_on_channel_14_is_not_taken() {
    assert_eq!(hunt_heard_on(14, false), Err(HuntEnd::NotHeard(1)));
}

#[test]
fn a_beacon_heard_near_the_channel_it_names_is_joined_there() {
    let target = beacon(&[&RSNE_PSK]);
    assert_eq!(hunt_heard_on(8, true), Ok((target.clone(), 6)), "named 6, heard on 8");
    assert_eq!(hunt_heard_on(11, true), Ok((target, 11)), "too far: where it was heard");
}
