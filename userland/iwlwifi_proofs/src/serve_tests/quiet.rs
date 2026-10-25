// NONOS Operating System (AGPL-3.0-or-later)
//! No scan transmits. Background sweeps over every channel the world domain
//! allows, and a hunt for a network not on the air, send the firmware only
//! scan requests forced passive, with no probe request and no SSID in them,
//! and queue no frame of the station's own.

use nonos_wifi_core::bss::Candidates;
use nonos_wifi_core::regdom::world;
use nonos_wifi_core::rsn::JoinPolicy;

use super::scripted::{completion, heard, other_network};
use crate::gen3::join::hunt::{hunt, HuntEnd};
use crate::gen3::sweep::{Sweep, Tick};
use crate::gen3_model::{reply, Seen};
use crate::join_rig::{rig, Security, Station, Switches};

const FORCE_PASSIVE: u16 = 1 << 11;

#[test]
fn sweeps_and_hunts_ask_only_for_passive_scans_and_queue_nothing() {
    let r = rig(Security::Wpa2, Switches::default());
    r.model.s.borrow_mut().responder = Some(Box::new(|c: &Seen| match (c.group, c.cmd) {
        (0x1, 0x0D) => vec![reply(c, vec![]), heard(&other_network(), 1), completion(c, 1)],
        _ => vec![reply(c, vec![])],
    }));
    let channels: Vec<u8> = (0..=255).filter(|&c| world(c).is_some()).collect();
    let mut st = Station::new();
    crate::join_rig::with_fw(&r, &mut st, |fw| {
        let mut sweep = Sweep::new();
        for now in 0..3 {
            let mut tick = sweep.start(fw.dev, fw.clock, &channels, now, &mut |_| {});
            for _ in 0..4 {
                if tick == Ok(Tick::Running) {
                    tick = Ok(sweep.pump(fw.dev, now, &mut |_| {}));
                }
            }
            assert!(matches!(tick, Ok(Tick::Completed(_))), "sweep {now}: {tick:?}");
        }
        let mut found = Candidates::new(b"absent", JoinPolicy::ANY);
        let got = hunt(fw.dev, fw.clock, &mut sweep, &channels, &mut found, &mut || 0, &mut |_| {});
        assert_eq!(got, Err(HuntEnd::NotHeard(1)));
    });
    assert!(r.commands().iter().all(|&c| c == (0x1, 0x0D)), "scan requests alone");
    let scans = r.command_payloads(0x1, 0x0D);
    assert_eq!(scans.len(), 4, "three sweeps and the hunt's");
    for s in &scans {
        assert_ne!(u16::from_le_bytes([s[8], s[9]]) & FORCE_PASSIVE, 0, "forced passive");
        assert_eq!(s[45], channels.len() as u8, "every allowed channel");
        assert!(s[596..].iter().all(|&b| b == 0), "no probe request, no SSID");
    }
    assert!(r.sent().is_empty(), "no frame queued");
}
