// NONOS Operating System (AGPL-3.0-or-later)
//! A join under the network's own address: the link and MAC contexts are
//! removed and added again under it before the join's first command, the
//! join runs under it, and a firmware that fails there ends the join before
//! any context of the join's own exists.

use nonos_wifi_core::rsn::JoinPolicy;

use super::rig::{JOIN_COMMANDS, LINK, MAC};
use crate::gen3::join::address::join_as;
use crate::gen3::join::bss::SetupError;
use crate::gen3::join::exchange::Progress;
use crate::gen3::join::run::JoinEnd;
use crate::join_rig::*;

fn run(r: &Rig) -> Result<(), JoinEnd> {
    let beacon = r.ap.borrow().beacon();
    let mut st = Station::new();
    let mut end = Ok(());
    with_fw(r, &mut st, |fw| {
        let mut p = Progress::default();
        // The rig's access point keys the handshake to STA, the address the
        // join is asked to run under.
        end = join_as(fw, &request(JoinPolicy::ANY), &beacon, 6, 0x3, 0x3, &mut p).map(|_| ());
    });
    end
}

#[test]
fn the_contexts_are_added_again_under_the_networks_address_before_the_join() {
    let r = rig(Security::Wpa2, Switches::default());
    assert_eq!(run(&r), Ok(()), "joined under it");
    let want: Vec<_> = [LINK, MAC, MAC, LINK].into_iter().chain(JOIN_COMMANDS).collect();
    assert_eq!(r.commands()[..want.len()], want[..]);
    let (links, macs) = (r.command_payloads(0x3, 0x09), r.command_payloads(0x3, 0x08));
    let word = |c: &[u8], at: usize| u32::from_le_bytes(c[at..at + 4].try_into().unwrap());
    assert_eq!((word(&links[0], 0), word(&links[0], 12)), (3, 0xFFFF_FFFF), "link removed");
    assert_eq!((word(&macs[0], 0), word(&macs[0], 4)), (0, 3), "MAC 0 removed");
    assert_eq!((word(&macs[1], 4), &macs[1][12..18]), (1, &STA[..]), "MAC added under it");
    assert_eq!((word(&links[1], 0), &links[1][16..22]), (1, &STA[..]), "link added under it");
}

#[test]
fn a_firmware_that_fails_the_readdress_ends_the_join_before_its_own_contexts() {
    let r = rig(Security::Wpa2, Switches::default());
    r.model.s.borrow_mut().fw_error_after = Some(0);
    match run(&r) {
        Err(JoinEnd::Setup(SetupError::Command(c))) => assert_eq!((c.group, c.cmd), LINK),
        other => panic!("{other:?}"),
    }
    assert!(!r.commands().contains(&(0x1, 0x08)), "no PHY context");
    assert!(r.sent().is_empty(), "nothing was sent");
}
