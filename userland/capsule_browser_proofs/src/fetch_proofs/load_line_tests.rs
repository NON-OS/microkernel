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

//! The first paint's bounded wait for stylesheets, and the line each page
//! load leaves at about:loads and on the serial console.

use super::fetch_fixtures::url_of;
use crate::browser::fetch::nav_trace::{line, size, stage, status_line, stopped};
use crate::browser::fetch::style_hold::{StyleHold, STYLE_HOLD_MS};
use crate::browser::fetch::types::{Fetch, Phase};

#[test]
fn the_style_wait_ends_once_and_on_time() {
    let mut h = StyleHold::from(1_000);
    assert!(!h.lapse(1_000 + STYLE_HOLD_MS - 1));
    assert!(!h.over);
    assert!(h.lapse(1_000 + STYLE_HOLD_MS));
    assert!(h.over);
    assert!(!h.lapse(1_000_000), "it ends once");
    assert!(!StyleHold::default().lapse(i64::MAX), "no wait, nothing to end");
}

#[test]
fn a_stop_keeps_the_stage_it_stopped_in() {
    let mut f = Fetch::new(url_of("https://github.com/"), 3, Phase::TlsFlight, 0);
    f.stop("tls handshake refused");
    f.stop("a second reason");
    assert_eq!(f.stopped_in, Some(Phase::TlsFlight));
    f.tls_alert = Some(40);
    let what = stopped(&f, "The site refused the secure connection.");
    assert_eq!(what, "tls failed (a second reason, alert 40): The site refused the secure connection.");
}

#[test]
fn the_line_names_host_time_bytes_and_network() {
    let mut f = Fetch::new(url_of("https://nonos.software/"), 3, Phase::ReadBody, 2_000);
    f.received = 312 * 1024;
    let l = line(&f, 14_500, "HTTP/1.1 200 OK (312 KB), page built in 900 ms");
    assert!(l.starts_with("[BROWSER] nonos.software: HTTP/1.1 200 OK"), "{l}");
    assert!(l.contains("after 12 s, 312 KB in, over "), "{l}");
    assert!(l.ends_with('\n'));
}

#[test]
fn stages_sizes_and_status_lines_read_plainly() {
    assert_eq!(stage(Phase::SocksConnect), "proxy");
    assert_eq!(stage(Phase::Connecting), "connect");
    assert_eq!(stage(Phase::ReadBody), "response");
    assert_eq!(size(512), "512 B");
    assert_eq!(size(2048), "2 KB");
    assert_eq!(size(3 * 1024 * 1024 + 512 * 1024), "3.5 MB");
    assert_eq!(status_line(b"HTTP/1.1 404 Not Found\r\nA: b\r\n\r\n"), "HTTP/1.1 404 Not Found");
    assert_eq!(status_line(b""), "");
}

#[test]
fn late_sheets_restyle_at_most_once_a_gap() {
    use crate::browser::fetch::style_hold::RESTYLE_GAP_MS;
    let mut h = StyleHold::from(0);
    assert!(h.lapse(STYLE_HOLD_MS));
    assert!(!h.restyle(STYLE_HOLD_MS + RESTYLE_GAP_MS), "nothing landed");
    h.stale = true;
    assert!(!h.restyle(STYLE_HOLD_MS + RESTYLE_GAP_MS - 1), "too soon after the first paint");
    assert!(h.restyle(STYLE_HOLD_MS + RESTYLE_GAP_MS));
    h.stale = true;
    assert!(!h.restyle(STYLE_HOLD_MS + RESTYLE_GAP_MS + 10));
    assert!(h.restyle(STYLE_HOLD_MS + 2 * RESTYLE_GAP_MS));
    assert!(!h.stale);
}
