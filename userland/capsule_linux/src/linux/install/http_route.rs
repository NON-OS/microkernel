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


//! A request for an install, over the Anyone network (`Route::for_installs`),
//! whatever network the browser uses: Nym's exits rotate and end a long
//! stream part way, Anyone's circuits carry a whole package. The mirror is
//! named, and the Anyone exit resolves the name, so an install names nothing
//! in the clear. On a cold boot Anyone takes minutes to build its first
//! circuit, so an install waits for it, up to three minutes, saying how far
//! it has got (the fetcher's own rule, `anyone_wait`). A network that does
//! not come up, or a stream it cannot open, is a refusal with its reason; no
//! failure is answered by trying another way. The reading is route_read.rs.

use alloc::vec::Vec;

use nonos_route_link::{Route, RouteStream};

use nonos_libc::{mk_idle_ms, mk_uptime_ms};

use super::anyone_wait::{decide, line, read, Heard, Wait, GAP_MS, STATUS_ASK};
use super::route_read::{read_reply, Reply, Source};

struct Stream(RouteStream);

impl Source for Stream {
    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str> {
        self.0.read_wait(into, wait_ms)
    }

    fn ended(&self) -> bool {
        self.0.ended()
    }
}

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
    let mut rx = [0u8; 16];
    let n = nonos_libc::mk_ipc_call_timeout(port as u64, [STATUS_ASK].as_ptr(), 1, rx.as_mut_ptr(), rx.len(), 2_000);
    match usize::try_from(n) {
        Ok(n) if n > 0 => (read(&rx[..n.min(rx.len())]), true),
        _ => (None, false),
    }
}

/// The reply to `request`, sent to `host`:`port` on the chosen route, read
/// until `done` says it is whole. None when the route is down, the stream
/// does not open or breaks before anything came, or more than `max` bytes
/// come.
pub(super) fn exchange(
    host: &str,
    port: u16,
    request: &[u8],
    max: usize,
    done: &dyn Fn(&[u8]) -> bool,
) -> Option<Vec<u8>> {
    /* Nothing reaches a network from a family that holds a model. */
    if crate::linux::file::models::held() {
        return refused(host, "this family holds a model");
    }
    let mut stream = match RouteStream::connect(Route::for_installs(), host, port) {
        Ok(s) => Stream(s),
        Err(why) => return refused(host, why),
    };
    if let Err(why) = stream.0.write_all(request) {
        return refused(host, why);
    }
    match read_reply(&mut stream, max, done) {
        Reply::Got(bytes) => Some(bytes),
        Reply::Nothing => refused(host, "nothing came back"),
        Reply::TooLarge => refused(host, "the reply is larger than any index or package"),
    }
}

/// Why `host` gave nothing, on the log, as raw.rs says it for a socket.
fn refused<T>(host: &str, why: &str) -> Option<T> {
    let line = alloc::format!("[LINUX] mirror {host}: {why}\n");
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    None
}
