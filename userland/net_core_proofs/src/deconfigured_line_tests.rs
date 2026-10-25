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

//! smoltcp's DHCP client reports Deconfigured on its first poll, before any
//! lease: net.core says it is asking, and says a lease dropped only after
//! one was held.

use crate::deconfigured_line::{line, ASKING, DROPPED};

const HANDLER: &str =
    include_str!("../../capsule_net_core/src/iface/dhcp/handle_deconfigured.rs");

#[test]
fn a_first_deconfigured_names_no_drop() {
    assert_eq!(line(false), ASKING);
    assert!(!core::str::from_utf8(line(false)).unwrap().contains("dropped"));
}

#[test]
fn one_after_a_held_lease_says_it_dropped() {
    assert_eq!(line(true), DROPPED);
    assert!(DROPPED.starts_with(b"[NET-CORE] lease dropped"));
}

#[test]
fn the_handler_reads_the_lease_before_clearing_it_and_says_the_rule() {
    let held = HANDLER.find("let held = state::lease().is_some();").expect("held never read");
    let cleared = HANDLER.find("state::set_lease(None);").expect("lease never cleared");
    assert!(held < cleared, "the lease is read after it is cleared, so always none");
    assert!(HANDLER.contains("let line = deconfigured_line::line(held);"));
    assert!(!HANDLER.contains("b\"[NET-CORE] lease dropped"), "a line said outside the rule");
}
