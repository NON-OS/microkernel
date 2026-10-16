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

//! ACL buffers held per link (Vol 4, Part E, 4.1.1): a link that ends gives
//! back all it held, a count for a link frees no more than it holds, and a
//! controller that never counts anything back stops only its own sends.

use crate::sim::config::Config;
use crate::sim::events::completed;
use crate::sim::link_events::disconnected;
use crate::sim::stack::{stack, Stack};
use nonos_bt_hci::HciError;

fn up() -> Stack {
    let mut s = stack(Config::default());
    s.bring_up().unwrap();
    s
}

#[test]
fn a_link_that_ends_gives_back_every_buffer_it_held() {
    let mut s = up();
    s.send_pdu(0x0040, &[0; 54]).unwrap();
    s.send_pdu(0x0041, &[0; 4]).unwrap();
    assert_eq!(s.acl_free(), 1);
    s.transport().pipes().send_event(&disconnected(0x0040, 0x13));
    assert!(s.next_event().unwrap().is_some(), "the caller still hears the link end");
    assert_eq!(s.acl_free(), 3);
    s.transport().pipes().send_event(&completed(0x0040, 2));
    assert_eq!(s.next_event(), Ok(None));
    assert_eq!(s.acl_free(), 3, "nothing more for the ended link");
    s.transport().pipes().send_event(&completed(0x0041, 4));
    assert_eq!(s.next_event(), Ok(None));
    assert_eq!(s.acl_free(), 4, "the other link's one, and no more");
}

#[test]
fn a_disconnection_that_failed_gives_back_nothing() {
    let mut s = up();
    s.send_pdu(0x0040, &[0; 4]).unwrap();
    // Disconnection Complete with status Command Disallowed (7.7.5).
    s.transport().pipes().send_event(&[0x05, 4, 0x0c, 0x40, 0x00, 0x13]);
    assert!(s.next_event().unwrap().is_some());
    assert_eq!(s.acl_free(), 3);
}

#[test]
fn buffers_never_counted_back_stop_only_sends_and_end_with_the_link() {
    let mut s = up();
    for _ in 0..4 {
        s.send_pdu(0x0040, &[0; 4]).unwrap();
    }
    for _ in 0..100 {
        assert_eq!(s.send_pdu(0x0040, &[0; 4]), Err(HciError::NoCredit));
    }
    assert_eq!(s.transport().pipes().bulk_out.len(), 4, "nothing sent past the buffers");
    s.link_gone(0x0040);
    assert_eq!(s.acl_free(), 4);
    s.link_gone(0x0040);
    assert_eq!(s.acl_free(), 4, "never past the controller's count");
}
