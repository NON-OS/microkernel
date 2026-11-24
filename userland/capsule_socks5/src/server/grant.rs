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

//! The grant a Linux personality's guest family is served on
//! (userland/capsule_linux/design/network-grant.md). The kernel lets a
//! ForeignExec holder without Network send here and to net.anon alone; here
//! such a sender is served only on a grant for its live family, asked for
//! with the GRANT_POLICY or GRANT_BOX marker and decided by the grant crate
//! on the kernel's facts, bound to that family's generation. Each tunnel it
//! opens goes only where its grant reaches (may_open, before open_tunnel in
//! feed.rs): a box's own destinations, or anywhere on a policy's. A sender
//! holding Network is served as it always was.
//!
//! A box grant is decided from the personality's argv as the kernel set it.
//! The argv is read only to find the box and is never logged or kept here:
//! the row holds the box's destinations and nothing else of it.

use alloc::vec;
use alloc::vec::Vec;

use linux_net_grant::wire::{GRANTED, NOT_GRANTED, SOCKS_GRANT_BOX, SOCKS_GRANT_POLICY};
use linux_net_grant::{decide, Ask, Grants, KernelFacts, Refusal};
use nonos_libc::mk_uptime_ms;
use spin::Mutex;

use crate::conn::Dest;

/// How often the rows of ended families are dropped.
const REAP_GAP_MS: i64 = 2000;

static GRANTS: Mutex<Grants> = Mutex::new(Grants::new());

/// The answer this request gets here, or None to serve it as ever.
pub fn gate(sender: u32, request: &[u8]) -> Option<Vec<u8>> {
    let ask = match request.split_first() {
        Some((&SOCKS_GRANT_POLICY, policy)) => Ask::Policy(policy),
        Some((&SOCKS_GRANT_BOX, _)) => Ask::Box,
        _ => {
            return (!GRANTS.lock().admits(sender, &KernelFacts)).then(|| vec![NOT_GRANTED]);
        }
    };
    let made =
        decide(sender, ask, &KernelFacts).and_then(|(g, n)| GRANTS.lock().insert(sender, n, g));
    Some(match made {
        Ok(()) => vec![GRANTED],
        Err(r) => vec![NOT_GRANTED, Refusal::code(r)],
    })
}

/// Whether `sender` may open a tunnel to `dest`: its family's grant reaches
/// it, or it holds Network. An address goes as its dotted form, a name as it
/// is, as the personality names them.
pub fn may_open(sender: u32, dest: &Dest) -> bool {
    let (host, port) = match dest {
        Dest::V4([a, b, c, d], port) => (alloc::format!("{a}.{b}.{c}.{d}").into_bytes(), *port),
        Dest::V6(_, port) => (Vec::new(), *port),
        Dest::Domain { name, len, port } => (name[..usize::from(*len)].to_vec(), *port),
    };
    GRANTS.lock().may_reach(sender, &host, port, &KernelFacts)
}

/// Drop the rows of ended families, at most once a REAP_GAP_MS.
pub fn reap_due(last: &mut i64) {
    let now = mk_uptime_ms();
    if *last != i64::MIN && now >= *last && now - *last < REAP_GAP_MS {
        return;
    }
    *last = now;
    GRANTS.lock().reap(&KernelFacts);
}
