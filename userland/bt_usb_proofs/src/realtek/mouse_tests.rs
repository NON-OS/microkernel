// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! On the patched HP controller, a BLE mouse pairs by Just Works with
//! nothing shown; its motion and buttons are posted, and Settings shows it
//! connected and how it paired. With no controller the page says so.

use super::board_env::BoardEnv;
use super::drive::one_link;
use super::panel::key::BtKey;
use super::panel::made;
use super::rom::patched;
use crate::air::Act;
use crate::fixture::air;
use crate::link::End;
use crate::smp_sim::Peer;
use nonos_bt_hogp::report::Input;
use nonos_bt_proto::{Asked, Link};
use nonos_bt_smp::method::IoCap;

const MOUSE: u16 = 16;

#[test]
fn a_mouse_pairs_by_just_works_and_moves_the_pointer() {
    let (mut host, ours, mut env, _) = patched(Peer::new(IoCap::NoInputNoOutput, false));
    env.look();
    let now = env.t.now;
    env.settings.press(&mut env.board, BtKey::Pair, now);
    air(&mut host).script.extend([Act::Notify(MOUSE, vec![0x01, 5, 0xfb]), Act::Drop(0x08)]);
    assert_eq!(one_link(&mut host, &ours, &mut env), End::Lost(0x08));
    let moved = [Input::Motion { dx: 5, dy: -5 }, Input::ButtonDown(1), Input::ButtonUp(1)];
    assert_eq!(env.t.input, moved);
    assert!(env.t.passkeys.is_empty(), "Just Works shows nothing");
    let seen = &env.settings.seen;
    assert!(seen.iter().all(|s| s.2 == Asked::None), "the page asked the user something");
    assert!(seen.iter().any(|s| s.1 == "Keyboard and mouse connected"));
    let p = &env.settings.panel;
    assert_eq!(made(&p.bonds[0]), "Paired without a check: Just Works");
    assert!(!p.has(Link::KEYS_POST), "a Just Works link posts no keys");
}

#[test]
fn with_no_controller_the_page_says_so() {
    let mut env = BoardEnv::new(Default::default());
    env.look();
    assert_eq!(env.settings.seen[0].0, "No Bluetooth controller found");
    assert_eq!(env.settings.seen[0].1, "No Bluetooth controller found");
}
