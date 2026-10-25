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

//! What the rail's rows read before anything was measured, and the formatters
//! for the figures a later read fills in.

use crate::paint::rail_addr::{ipv4_into, ipv4_pfx, ipv6_into, ipv6_str};
use crate::paint::rail_fmt::{bps_into, load3_into, mib_into, pct, uptime};
use crate::paint::rail_metric::{one, pair, DASH};
use crate::rail::metrics::Sample;

/// The first frame draws from the empty sample: every sourced figure is a dash,
/// and only the counters that start at a real zero show one.
pub fn assert_an_empty_sample_reads_as_dashes() {
    let s = Sample::EMPTY;
    let mut b = [0u8; 64];
    assert!(s.live().is_empty());
    let p = s.procs[0];
    assert_eq!((p.pid, p.cpu_pct, p.run_ticks), (0, 0, 0));
    assert_eq!((s.total_ticks, s.mem_total_kb), (0, 0));
    assert_eq!(pct(&mut b, s.cpu_pct), "0%");
    assert_eq!(uptime(&mut b, s.uptime_ms), "0h 0m 0s");
    assert_eq!(one(&mut b, s.load_avg, load3_into), DASH);
    assert_eq!(pair(&mut b, s.mem.used_kb, s.mem.total_kb, mib_into), "— / —");
    assert_eq!(pair(&mut b, s.disk.used_kb, s.disk.total_kb, mib_into), "— / —");
    let n = s.net;
    assert!(!n.up);
    assert_eq!(n.name_str(), "net0");
    assert_eq!(ipv4_pfx(&mut b, n.ipv4, n.prefix_len), DASH);
    assert_eq!(one(&mut b, n.gateway, ipv4_into), DASH);
    assert_eq!(ipv6_str(&mut b, n.ipv6), DASH);
    assert_eq!(pair(&mut b, n.rx_bps, n.tx_bps, bps_into), "— / —");
}

/// The figures those rows show once a read has a value for them.
pub fn assert_known_figures_are_written() {
    let mut b = [0u8; 64];
    assert_eq!(pct(&mut b, 57), "57%");
    assert_eq!(uptime(&mut b, 3_723_999), "1h 2m 3s");
    let n = bps_into(&mut b, 1500);
    assert_eq!(core::str::from_utf8(&b[..n]).unwrap(), "1500 B/s");
    let mut ip = [0u8; 16];
    ip[..4].copy_from_slice(&[0x20, 0x01, 0x0d, 0xb8]);
    ip[15] = 0xab;
    let n = ipv6_into(&mut b, ip);
    let want = "2001:0db8:0000:0000:0000:0000:0000:00ab";
    assert_eq!(core::str::from_utf8(&b[..n]).unwrap(), want);
}
