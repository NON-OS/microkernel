// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! On the patched HP controller, a BLE keyboard pairs by the passkey the
//! Settings page shows, types, and gives its battery; the page reads each
//! step from the capsule's NBTC, and the log never carries the passkey.

use super::drive::one_link;
use super::panel::key::BtKey;
use super::panel::{level_of, made};
use super::rom::patched;
use crate::air::Act;
use crate::fixture::air;
use crate::link::End;
use crate::smp_sim::Peer;
use nonos_bt_hogp::report::Input;
use nonos_bt_proto::ops::E_OK;
use nonos_bt_proto::Asked;
use nonos_bt_smp::method::IoCap;

const KEYS: u16 = 11;
const LEVEL: u16 = 23;

fn key(code: u8) -> [Act; 2] {
    [Act::Notify(KEYS, vec![0, 0, code, 0, 0, 0, 0, 0]), Act::Notify(KEYS, vec![0; 8])]
}

#[test]
fn a_keyboard_pairs_by_the_shown_passkey_types_and_gives_its_battery() {
    let (mut host, ours, mut env, _) = patched(Peer::new(IoCap::KeyboardOnly, true));
    env.look();
    assert_eq!(env.settings.seen[0].0, "Controller ready");
    let now = env.t.now;
    assert_eq!(env.settings.press(&mut env.board, BtKey::Pair, now), E_OK);
    let script = &mut air(&mut host).script;
    script.extend(key(0x0b).into_iter().chain(key(0x0c)));
    script.extend([Act::Notify(LEVEL, vec![80]), Act::Drop(0x13)]);
    assert_eq!(one_link(&mut host, &ours, &mut env), End::Lost(0x13));
    // h, i: typed on the keyboard, posted because the passkey protected it.
    let typed =
        [Input::KeyDown(0x0b), Input::KeyUp(0x0b), Input::KeyDown(0x0c), Input::KeyUp(0x0c)];
    assert_eq!(env.t.input, typed);
    let seen = &env.settings.seen;
    let shown: Vec<u32> =
        seen.iter().filter_map(|s| if let Asked::Show(n) = s.2 { Some(n) } else { None }).collect();
    assert_eq!(shown.first(), env.t.passkeys.first(), "the page showed the passkey pairing used");
    assert_eq!(env.t.passkeys.len(), 1);
    let order =
        ["Looking for devices", "Connecting", "Securing the link", "Keyboard and mouse connected"];
    let mut at = 0;
    for want in order {
        at += seen[at..].iter().position(|s| s.1 == want).unwrap_or_else(|| panic!("never {want}"));
    }
    let p = &env.settings.panel;
    assert_eq!(p.bond_count, 1);
    assert_eq!(made(&p.bonds[0]), "Paired with a passkey or number checked");
    assert_eq!(level_of(p, 0), Some(80), "the battery the keyboard gave");
    let digits = format!("{:06}", env.t.passkeys[0]);
    assert!(env.t.lines.iter().all(|l| !l.contains(&digits)), "the log carries the passkey");
}
