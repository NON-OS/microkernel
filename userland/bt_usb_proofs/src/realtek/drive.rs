// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! One link as run/links.rs runs it, the board's state set at each step
//! as it sets it, and Settings looking at each.

use super::board_env::BoardEnv;
use crate::fixture::Stack;
use crate::link::us::Us;
use crate::link::{begin, find_hid, serve, End};
use crate::nbtc::ops::{CONNECTING, SCANNING, SECURING};
use nonos_bt_hci::host::{Connect, DEFAULT_LINK, DEFAULT_SCAN};
use nonos_bt_wire::addr::{AddrType, BdAddr};

pub fn one_link(host: &mut Stack, ours: &BdAddr, env: &mut BoardEnv) -> End {
    let us = Us { kind: AddrType::Public, addr: *ours, identity: None };
    env.board.state = SCANNING;
    env.look();
    let peer = find_hid(host, env, &us).unwrap().expect("no HID device heard");
    env.board.state = CONNECTING;
    env.look();
    let (peer_type, peer, scan, link) = (peer.kind, peer.addr, DEFAULT_SCAN, DEFAULT_LINK);
    let link = host.connect(&Connect { scan, peer_type, peer, link }).unwrap();
    env.board.state = SECURING;
    env.look();
    let mut s = begin(host, &link, &us, env).unwrap().expect("pairing did not start");
    let end = serve(host, &mut s, env).unwrap();
    env.board.ended();
    env.look();
    end
}
