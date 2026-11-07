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

#![no_main]

use std::collections::HashMap;

use libfuzzer_sys::fuzz_target;
use nonos_shell::builtin::dns::is_host;
use nonos_shell::builtin::ping::PING_COUNT;
use nonos_shell::builtin::service::valid;
use nonos_shell::host::{Echo, Resolve};
use nonos_shell::session::Session;
use nonos_shell_proofs::kit::{Lines, Script};

fn u64_at(b: &[u8], i: usize) -> u64 {
    let mut w = [0u8; 8];
    for (k, x) in w.iter_mut().enumerate() {
        *x = b.get(i + k).copied().unwrap_or(0);
    }
    u64::from_le_bytes(w)
}

/// The host a session line runs against: its answers chosen by the first
/// bytes, so a line meets a registry, a resolver and echoes of every kind.
fn host(b: &[u8]) -> Script {
    let sel = b.first().copied().unwrap_or(0);
    let mut reg = HashMap::new();
    reg.insert(b"nsh.attest".to_vec(), (4486, 15));
    reg.insert(b"net.sockets".to_vec(), (4300, 9));
    let resolve = match sel % 6 {
        0 => Some(Resolve::Addr([10, 0, 2, 2])),
        1 => Some(Resolve::Refused("not Direct")),
        2 => Some(Resolve::NoName),
        3 => Some(Resolve::NoDns),
        4 => Some(Resolve::Unavailable),
        _ => None,
    };
    let echoes = (0..8)
        .map(|i| match (sel >> 3).wrapping_add(i) % 8 {
            0 => Echo::Reply(u64_at(b, 1 + usize::from(i))),
            1 => Echo::Timeout,
            2 => Echo::Unreachable,
            3 => Echo::NoRoute,
            4 => Echo::NotReady,
            5 => Echo::Failed,
            6 => Echo::Refused("not Direct"),
            _ => Echo::Reply(u64::from(i)),
        })
        .collect();
    Script {
        random: (sel & 0x40 != 0).then_some(sel),
        wall_ms: Some(u64_at(b, 1)),
        uptime_ms: Some(u64_at(b, 9)),
        services: (sel & 0x80 != 0).then_some(reg),
        resolve,
        echoes,
        ..Script::default()
    }
}

fuzz_target!(|data: &[u8]| {
    let (head, line) = data.split_at(data.len().min(17));
    let mut h = host(head);
    let mut out = Lines::default();
    Session::default().run_line(line, &mut out, &mut h);
    // Only a name the registry could hold is ever looked up, and only a
    // name DNS could carry is ever resolved.
    assert!(h.asked.iter().all(|n| valid(n)), "asked {:?}", h.asked);
    assert!(h.resolved.iter().all(|n| is_host(n)), "resolved {:?}", h.resolved);
    // Each ping sends its few echoes, numbered from one.
    assert!(h.echoed.iter().all(|&(_, seq)| (1..=PING_COUNT).contains(&seq)));
});
