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

//! Past net.anon's share for one caller, a connect is ENOBUFS.

use super::adopt::{quiet, ANON, GUEST, LIMITS, OWNED, TO};
use crate::linux::abi::errno::ENOBUFS;
use crate::linux::net::anon_answer::opened;
use crate::linux::net::anon_ops::{E_OK, E_TABLE_FULL};
use crate::linux::net::anon_stream;
use crate::linux::net::sock::{Domain, Proto, Socks};

#[test]
fn past_net_anons_share_for_one_caller_a_connect_is_enobufs() {
    /* net.anon gives one caller half its table: this capsule, for every guest. */
    assert!(LIMITS.contains("pub const STREAM_MAX: usize = 32;"));
    assert!(OWNED.contains("streams.iter().filter(|s| s.owner == owner).count() < max / 2"));
    let share = 32 / 2;
    quiet();
    let mut t = Socks::new();
    let mut held = 0usize;
    for n in 0..=share {
        let id = t.open(Domain::Inet, Proto::Stream, Some(GUEST));
        /* What net.anon answers this caller's open, as may_open decides it. */
        let sid = (n as u16 + 1).to_le_bytes();
        let answer = if held < share { (E_OK, &sid[..]) } else { (E_TABLE_FULL, &[][..]) };
        match opened(Some(answer)) {
            Ok(sid) => {
                assert_eq!(t.adopt_anon(id, ANON, sid, TO), Ok(()));
                held += 1;
            }
            Err(e) => {
                assert_eq!((n, e), (share, ENOBUFS), "the stream past the share, and only it");
                let s = t.get(id).expect("there");
                assert!(s.svc.is_none() && !s.connected, "nothing was adopted");
            }
        }
    }
    assert_eq!(held, share);
    assert!(anon_stream::closed().is_empty(), "no stream is closed by a refusal");
}
