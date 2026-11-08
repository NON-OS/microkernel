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
//
// Host-side harness for the rail's telemetry decoders and the figures it
// shows from them. Compiled with the host toolchain:
//   rustc --edition 2021 --test tests/rail_host.rs -o /tmp/rail_host && /tmp/rail_host
//
// The list geometry, its hit-test and the scrolled column now draw through the
// toolkit, which does not resolve outside cargo; they are proven in
// `terminal_line_proofs` (rail_*_tests.rs). The formatters are covered by
// `rail_fmt_host.rs`.

#[path = "../src/term"]
mod term {
    pub mod util {
        pub mod copy_into;
        pub mod format_u64;

        pub use copy_into::copy_into;
        pub use format_u64::format_u64;
    }
}

#[path = "../src/paint"]
mod paint {
    pub mod rail_fmt;

    pub mod rail_metric;

    pub mod rail_addr;
}

#[path = "../src/rail"]
mod rail {
    pub mod disk;
    pub mod disk_decode;
    pub mod mem;
    pub mod metrics;
    pub mod net;
    pub mod net_decode;
    pub mod value;
}

use paint::rail_addr::{ipv4_pfx, ipv6_str};
use paint::rail_metric::DASH;
use rail::disk::Disk;
use rail::disk_decode::{decode_usage, REPLY_LEN as DISK_REPLY_LEN};
use rail::mem::summarize;
use rail::metrics::{Proc, Sample};
use rail::net::Net;
use rail::net_decode::{decode_lease, HDR_LEN, REPLY_LEN};
use rail::value::Metric;

#[test]
fn the_lease_the_rail_shows_is_the_lease_the_decoder_read() {
    let mut b = [0u8; 64];
    let n = decode_lease(&lease(3, [10, 0, 2, 15], 24));
    assert_eq!(ipv4_pfx(&mut b, n.ipv4, n.prefix_len), "10.0.2.15/24");
    assert_eq!(ipv6_str(&mut b, n.ipv6), DASH, "no v6 stack, so no v6 row figure");
    assert_eq!(ipv6_str(&mut b, Metric::Known([0; 16])), "0000:0000:0000:0000:0000:0000:0000:0000");
}

fn proc_with(pid: u32, mem_kb: u64) -> Proc {
    Proc { pid, mem_kb, ..Proc::EMPTY }
}

fn lease(state: u8, ip: [u8; 4], prefix: u8) -> [u8; REPLY_LEN] {
    let mut rx = [0u8; REPLY_LEN];
    rx[HDR_LEN] = state;
    rx[HDR_LEN + 1..HDR_LEN + 5].copy_from_slice(&ip);
    rx[HDR_LEN + 5] = prefix;
    rx[HDR_LEN + 6..HDR_LEN + 10].copy_from_slice(&[10, 0, 2, 2]);
    rx
}

#[test]
fn memory_used_is_the_resident_sum_and_saturates() {
    let live = [proc_with(1, 4096), proc_with(2, 512), proc_with(3, 0)];
    assert_eq!(summarize(&live, 0).used_kb, Metric::Known(4608));
    let huge = [proc_with(1, u64::MAX), proc_with(2, 1024)];
    assert_eq!(summarize(&huge, 0).used_kb, Metric::Known(u64::MAX));
}

#[test]
fn an_empty_process_table_is_unknown_memory_rather_than_zero() {
    let m = summarize(&[], 0);
    assert_eq!(m.used_kb, Metric::Unknown);
}

#[test]
fn only_swap_has_no_source_at_all() {
    let m = summarize(&[proc_with(1, 8)], 2 * 1024 * 1024);
    assert_eq!(m.total_kb, Metric::Known(2 * 1024 * 1024));
    assert!(m.swap_used_kb == Metric::Unsupported);
}

#[test]
fn an_unreadable_memory_map_leaves_the_total_unmeasured_rather_than_absent() {
    let m = summarize(&[proc_with(1, 8)], 0);
    assert_eq!(m.total_kb, Metric::Unknown);
    assert!(m.total_kb != Metric::Unsupported);
}

#[test]
fn a_bound_lease_yields_the_real_address() {
    let n = decode_lease(&lease(3, [10, 0, 2, 15], 24));
    assert!(n.up && n.name_str() == "net0");
    assert_eq!(n.ipv4, Metric::Known([10, 0, 2, 15]));
    assert_eq!(n.prefix_len, Metric::Known(24));
    assert_eq!(n.gateway, Metric::Known([10, 0, 2, 2]));
}

#[test]
fn a_pre_bound_or_short_reply_leaves_the_interface_down() {
    assert_eq!(decode_lease(&lease(2, [10, 0, 2, 15], 24)), Net::DOWN);
    assert_eq!(decode_lease(&[0u8; 8]), Net::DOWN);
    assert!(!Net::DOWN.up && Net::DOWN.ipv4 == Metric::Unknown);
}

#[test]
fn the_figures_nonos_cannot_measure_stay_unsupported() {
    let s = Sample::EMPTY;
    let up = decode_lease(&lease(3, [1, 2, 3, 4], 8));
    assert!(
        up.ipv6 == Metric::Unsupported
            && up.rx_bps == Metric::Unsupported
            && up.tx_bps == Metric::Unsupported
    );
    assert!(s.disk.total_kb == Metric::Unsupported && s.disk.used_kb == Metric::Unknown);
    assert!(s.load_avg == Metric::Unknown, "load has a source, it is merely unread");
    assert_eq!(Metric::Known(7u32).value(), Some(7));
}

fn usage(status: i32, bytes: u64) -> [u8; DISK_REPLY_LEN] {
    let mut rx = [0u8; DISK_REPLY_LEN];
    rx[HDR_LEN..HDR_LEN + 4].copy_from_slice(&status.to_le_bytes());
    rx[HDR_LEN + 4..HDR_LEN + 8].copy_from_slice(&3u32.to_le_bytes());
    rx[HDR_LEN + 8..HDR_LEN + 16].copy_from_slice(&bytes.to_le_bytes());
    rx[HDR_LEN + 16..HDR_LEN + 20].copy_from_slice(&2048u32.to_le_bytes());
    rx
}

#[test]
fn a_usage_reply_yields_the_bytes_the_store_holds() {
    let d = decode_usage(&usage(0, 5 * 1024 * 1024));
    assert_eq!(d.used_kb, Metric::Known(5 * 1024));
    assert!(d.total_kb == Metric::Unsupported);
}

#[test]
fn an_empty_store_is_a_measured_zero_rather_than_a_gap() {
    assert_eq!(decode_usage(&usage(0, 0)).used_kb, Metric::Known(0));
}

#[test]
fn a_failed_or_short_usage_reply_leaves_the_store_unmeasured() {
    assert_eq!(decode_usage(&usage(-13, 4096)), Disk::UNKNOWN);
    assert_eq!(decode_usage(&usage(0, 4096)[..DISK_REPLY_LEN - 1]), Disk::UNKNOWN);
}
