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

//! Waiting for the Anyone network before an install downloads: up to three
//! minutes for a circuit, each step said once (the fetcher's own rule,
//! `anyone_wait`).

use nonos_libc::{mk_idle_ms, mk_ipc_call_timeout, mk_uptime_ms};

use super::anyone_wait::{decide, line, read, Heard, Wait, GAP_MS, STATUS_ASK};

/// Why an install cannot download now, or None once Anyone has a circuit;
/// waited for up to three minutes.
pub(super) fn down() -> Option<&'static str> {
    let since = mk_uptime_ms();
    let mut said = None;
    loop {
        let port = anon_port();
        let (heard, answered) = if port == 0 { (None, false) } else { ask(port) };
        match decide(port != 0, heard, answered, mk_uptime_ms() - since) {
            Wait::Go => return None,
            Wait::GiveUp => return Some(NOT_UP),
            Wait::Again(h) => {
                /* Each step once on the log, so a slow bootstrap is seen to move. */
                if said != Some(h) {
                    let line = alloc::format!("[LINUX] waiting for Anyone: {}\n", line(h));
                    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
                    said = Some(h);
                }
                mk_idle_ms(GAP_MS as u64);
            }
        }
    }
}

const NOT_UP: &str = "the Anyone network did not build a circuit within 3 minutes";

/* net.anon's port, 0 while it has not registered. */
fn anon_port() -> u32 {
    let (mut port, mut pid) = (0u32, 0u32);
    let name = b"net.anon";
    let rc = nonos_libc::mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    if rc < 0 || pid == 0 {
        0
    } else {
        port
    }
}

/* How far net.anon has got, and whether it answered the ask at all. */
fn ask(port: u32) -> (Option<Heard>, bool) {
    let (tx, mut rx) = ([STATUS_ASK], [0u8; 16]);
    let n = mk_ipc_call_timeout(port as u64, tx.as_ptr(), 1, rx.as_mut_ptr(), rx.len(), 2_000);
    match usize::try_from(n) {
        Ok(n) if n > 0 => (read(&rx[..n.min(rx.len())]), true),
        _ => (None, false),
    }
}
