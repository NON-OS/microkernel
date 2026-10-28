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

//! Nothing left behind: an offer the full table cannot keep is given no
//! record, and a host that names a connection the kernel refuses is told
//! to forget the contact.

use super::util::{chan, host, up, usual, HEARTBEAT, ROOT, SCSI};
use crate::constants::MAX_CHANNELS;
use crate::model::offer_bytes;
use crate::serve::step;
use crate::setup;

#[test]
fn an_offer_the_full_table_cannot_keep_is_given_no_record() {
    // Relids past the kernel's bound fill the core's table, not the
    // kernel's, which has room for the storage offer that follows.
    let (mut d, m) = up(vec![]);
    for i in 0..MAX_CHANNELS as u32 {
        m.send(&offer_bytes(&chan(3000 + i, HEARTBEAT, 0)));
    }
    m.send(&offer_bytes(&chan(9, SCSI, 0)));
    while step(&mut d) > 0 {}
    assert_eq!(d.offers.dropped, 1);
    assert!(nonos_libc::given().is_empty(), "no record for an offer not kept");
}

#[test]
fn a_refused_connection_is_unloaded_where_the_contact_was_made() {
    let m = host(usual());
    m.0.borrow_mut().named = 7;
    assert_eq!(setup::run(ROOT).err(), Some("host named a message connection the kernel refuses"));
    let s = m.0.borrow();
    assert!(s.unloaded, "the contact undone");
    assert_eq!(s.log.last(), Some(&(4, 16)), "UNLOAD on the contact's connection");
}
