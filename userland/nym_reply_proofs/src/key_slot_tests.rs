// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The gateway key in its one slot, driven through the paths the capsule
//! takes: registered on a link, kept when that link's gateway is set, wiped
//! where it lies when a gateway on another link is set or the link is lost.

use std::sync::Mutex;

use crate::state::key_slot::Link;
use crate::state::shared_key::{
    clear_gateway_shared_key, gateway_key_held, set_gateway_shared_key, with_gateway_key,
};
use crate::state::table::Table;
use crate::state::{Gateway, Transport};

/// The slot is one static: tests that reach it take this first.
pub(crate) static SLOT: Mutex<()> = Mutex::new(());

fn gateway(ip: [u8; 4], stream: u32) -> Gateway {
    Gateway { ip, port: 9000, stream, transport: Transport::WebSocket, identity: [3; 32] }
}

fn zeros_at(p: *const u8) -> bool {
    // SAFETY: `p` points into the static slot, which lives for the program.
    (0..32).all(|i| unsafe { core::ptr::read_volatile(p.add(i)) } == 0)
}

#[test]
fn the_key_is_kept_for_its_link_and_wiped_for_any_other() {
    let _held = SLOT.lock().unwrap_or_else(|e| e.into_inner());
    let mut t = Table::new();
    let a = gateway([10, 0, 0, 7], 4);
    set_gateway_shared_key(a.link(), &[0xC3; 32]);
    t.set_gateway(a);
    assert_eq!(with_gateway_key(|k| *k), Some([0xC3; 32]), "kept for its own link");
    let at = with_gateway_key(|k| k.as_ptr()).expect("held");
    t.set_gateway(gateway([10, 0, 0, 7], 5));
    assert!(!gateway_key_held(), "a new connection to the same gateway registers anew");
    assert!(zeros_at(at), "wiped where it lay");
    set_gateway_shared_key(Link { ip: [10, 0, 0, 8], port: 9000, stream: 1 }, &[0x5A; 32]);
    t.set_gateway(a);
    assert!(!gateway_key_held() && zeros_at(at), "another gateway's key goes");
}

#[test]
fn a_lost_link_wipes_the_key_where_it_lay() {
    let _held = SLOT.lock().unwrap_or_else(|e| e.into_inner());
    set_gateway_shared_key(gateway([10, 0, 0, 9], 2).link(), &[0x77; 32]);
    let at = with_gateway_key(|k| k.as_ptr()).expect("held");
    clear_gateway_shared_key();
    assert_eq!(with_gateway_key(|k| *k), None);
    assert!(zeros_at(at));
}
