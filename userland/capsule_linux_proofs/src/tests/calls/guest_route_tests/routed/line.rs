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

//! The line a person reads when a stream did not open: one whole line, the
//! network by name, a reason, and never the host or the port, for every
//! reason the capsule can give.

use crate::linux::net::anon_ops::*;
use crate::linux::net::guest_route::{Net, SOCKS_DOWN, UNREAD};
use crate::linux::net::routed::line::{line, word};
use crate::linux::net::routed::retry::{not_ready, refused};
use crate::pick::{ANYONE_DOWN, NYM_DOWN};
use crate::refusal::Proxy;

/// Every reason a stream that did not open can be given.
fn reasons() -> Vec<&'static str> {
    let mut all = vec![UNREAD, SOCKS_DOWN, ANYONE_DOWN, NYM_DOWN];
    let n = Proxy::Nym;
    all.extend([n.slow(), n.silent(), n.full(), n.garbled(), n.lost(), n.ended()]);
    all.extend((0u8..=8).map(|r| n.refused(r)));
    all.extend((0u16..64).filter_map(not_ready));
    all.extend([E_TABLE_FULL, E_BAD_LEN].map(|s| refused(Some(s))));
    all.push(refused(None));
    all
}

#[test]
fn the_line_names_the_network_and_its_reason_on_one_whole_line() {
    for net in [Net::Anyone, Net::Nym, Net::Unread] {
        for why in reasons() {
            let l = line(net, why);
            assert!(l.starts_with(&format!("[LINUX] net: {} could not open a stream: ", word(net))));
            assert!(l.ends_with(&format!("{why}\n")), "{l}");
            assert_eq!(l.matches('\n').count(), 1, "{l}");
            assert!(l.len() <= 256, "one write takes it whole: {l}");
        }
    }
    assert_eq!((word(Net::Anyone), word(Net::Nym)), ("Anyone", "Nym"));
}

#[test]
fn the_line_never_names_a_host_or_a_port() {
    for why in reasons() {
        let l = line(Net::Nym, why);
        assert!(!l.contains(':') || l.matches(':').count() == 2, "{l}");
        let number =
            l.as_bytes().windows(2).any(|w| w[0].is_ascii_digit() && w[1].is_ascii_digit());
        assert!(!number, "a port or an address on the line: {l}");
    }
    /* A reason is never a guest's bytes; a control byte would still not end the line. */
    assert_eq!(
        line(Net::Nym, "a\nb\rc\u{7f}é"),
        "[LINUX] net: Nym could not open a stream: a?b?c??\n"
    );
    assert!(line(Net::Nym, &"x".repeat(1000)).len() <= 256);
}
