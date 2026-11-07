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
//! The Wi-Fi and Network pages re-read the driver and the lease every
//! second, and each tick drew the whole window again (409810 pixels a second
//! on a development guest with Settings open) though nothing on the page
//! had changed. A tick asks for a frame now only when what the page shows
//! came back different (capsule_settings settings/tick_view.rs).

use crate::tick_view::reread;

/// A page's state: what it shows, and when it last asked, which it does not.
#[derive(Clone, Copy, Default)]
struct Page {
    link: Option<u32>,
    lease: Option<[u8; 4]>,
    asked_ms: Option<i64>,
}

fn shown(p: &Page) -> (Option<u32>, Option<[u8; 4]>) {
    (p.link, p.lease)
}

#[test]
fn a_tick_that_reads_back_the_same_asks_no_frame() {
    let mut p = Page { link: Some(3), lease: Some([10, 0, 2, 15]), asked_ms: Some(1000) };
    for now in [2000, 3000, 4000] {
        assert!(!reread(&mut p, shown, |p| p.asked_ms = Some(now)), "asked at {now} ms");
    }
    assert_eq!(p.asked_ms, Some(4000), "the read still ran");
}

#[test]
fn a_tick_that_reads_a_change_asks_a_frame() {
    let mut p = Page::default();
    assert!(reread(&mut p, shown, |p| p.link = Some(1)), "a link came up");
    assert!(reread(&mut p, shown, |p| p.lease = Some([10, 0, 2, 15])), "the lease bound");
    assert!(!reread(&mut p, shown, |p| p.lease = Some([10, 0, 2, 15])), "and stays");
    assert!(reread(&mut p, shown, |p| p.link = None), "the link went");
}

#[test]
fn every_change_of_a_shown_value_over_many_ticks_is_painted() {
    // A seeded walk: the page asks for exactly the frames in which what it
    // shows changed, never fewer.
    let mut p = Page::default();
    let mut x = 0x2545_F491_u32;
    let (mut asked, mut changed) = (0, 0);
    for tick in 0..2000_i64 {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        let before = shown(&p);
        let draw = reread(&mut p, shown, |p| {
            p.asked_ms = Some(tick * 1000);
            if x % 7 == 0 {
                p.link = Some(x % 3);
            }
            if x % 11 == 0 {
                p.lease = (x % 2 == 0).then_some([10, 0, 2, (x % 4) as u8]);
            }
        });
        changed += usize::from(shown(&p) != before);
        asked += usize::from(draw);
        assert_eq!(draw, shown(&p) != before, "tick {tick}");
    }
    assert_eq!(asked, changed);
    assert!(asked > 0 && asked < 2000, "{asked} frames of 2000 ticks");
}

#[test]
fn the_network_pages_compare_what_they_show_and_not_when_they_asked() {
    const TICK: &str = include_str!("../../../capsule_settings/src/settings/page_tick.rs");
    assert!(TICK.contains(
        "Section::Wifi | Section::Network => reread(state, wifi_shown, refresh_wifi_status),"
    ));
    assert!(
        TICK.contains("reread(state, |s| s.lock_set, |s| s.lock_set = super::actions::lock_set())")
    );
    let shown = TICK.split("fn wifi_shown(").nth(1).expect("wifi_shown");
    let shown = shown.split("\n}\n").next().expect("its body");
    assert!(!shown.contains("net_polled_ms"), "the time it asked is not drawn");
    for field in [
        "w.driver",
        "d.stage",
        "d.stop",
        "w.link",
        "w.keeps",
        "w.remember",
        "d.datapath",
        "d.net",
        "d.station",
    ] {
        assert!(shown.contains(field), "{field}");
    }
    const STATUS: &str =
        include_str!("../../../capsule_settings/src/settings/state/wifi/status.rs");
    let read = STATUS.split("pub fn refresh_wifi_status(").nth(1).expect("refresh");
    let read = read.split("\n}\n").next().expect("its body");
    for field in [
        "state.wifi.driver",
        "state.wifi_diag.stage",
        "state.wifi_diag.stop",
        "state.wifi.link",
        "state.wifi.keeps",
        "state.wifi.remember",
        "state.wifi_diag.datapath",
    ] {
        assert!(
            read.contains(&format!("{field} =")) || read.contains(&format!("{field} &=")),
            "{field} is read and shown"
        );
    }
    let net = STATUS.split("fn poll_net(").nth(1).expect("poll_net");
    assert!(net.contains("state.wifi_diag.net = net_status();"));
    assert!(net.contains("state.wifi_diag.station = station_source("));
    assert!(net.contains("state.wifi_diag.net_polled_ms = Some(now);"), "changes each second");
}
