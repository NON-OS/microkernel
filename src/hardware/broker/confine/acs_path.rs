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

//! The hops from a device to the root complex, pure (K11). kernel_proofs::
//! acs_rule.

extern crate alloc;

use alloc::vec::Vec;

use super::acs_rule::{FUNCTION, PORT};
use super::acs_tree::{with_below, Bdf, Func};

/// One hop: the function whose ACS counts and what it must redirect; the
/// peers that ACS keeps apart; and those nothing can, sharing a
/// conventional bus below it.
#[derive(Clone, Debug)]
pub(crate) struct Hop {
    pub at: Bdf,
    pub needed: u16,
    pub peers: Vec<Bdf>,
    pub always: Vec<Bdf>,
}

/// Bridges deeper than this are taken for a loop the table cannot hold.
const MAX_HOPS: usize = 32;

/// From `at` up: its own function, then each bridge above it: a port's
/// peers are the rest of its bus, a root port's its other functions. None
/// when the path does not end.
pub(crate) fn path(at: Bdf, all: &[Func]) -> Option<Vec<Hop>> {
    let others = |f: &Func| f.at.0 == at.0 && f.at.1 == at.1 && f.at != at;
    let mut hops = alloc::vec![Hop {
        at,
        needed: FUNCTION,
        peers: with_below(all, others),
        always: Vec::new()
    }];
    let mut below = at;
    for _ in 0..MAX_HOPS {
        let Some(port) = all.iter().find(|f| f.buses.is_some_and(|(s, _)| s == below.0)) else {
            return Some(hops);
        };
        let p = port.at;
        // On the root bus, peer to peer goes through the root complex, which
        // is not assumed; a root port's own other functions still count.
        let root = !all.iter().any(|f| f.buses.is_some_and(|(s, _)| s == p.0));
        let peers = with_below(all, |f| f.at.0 == p.0 && f.at != p && (!root || f.at.1 == p.1));
        let shared = |f: &Func| port.conventional && f.at.0 == below.0 && f.at.1 != below.1;
        let always = with_below(all, shared);
        hops.push(Hop { at: p, needed: PORT, peers, always });
        below = p;
    }
    None
}
