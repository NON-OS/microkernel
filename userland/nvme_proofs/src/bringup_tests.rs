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

//! The decisions a real laptop's SSD exercises and QEMU never did: the
//! namespace taken from the active namespace list, the wait before the reset
//! clears CC.EN on a controller firmware left mid-enable, waits on a clock
//! that fails, 64-bit registers moved as two dwords, which of several
//! controllers is served, the status a failed admin command is named by, and
//! the console line that says so. Each runs the driver's own code.

use core::cell::Cell;

use crate::admin::{
    await_ready_before_disable, disable_start, first_active_nsid, lists_active_namespaces,
    pre_disable_step, wait_ready, Completion, ControllerIdentity, DisableStart, NamespaceIdentity,
    PreDisableStep, SmartHealth, FALLBACK_NSID,
};
use crate::budget::Budget;
use crate::choice::{choose, settled, Choice, Outcome};
use crate::constants::{CC_EN, CSTS_CFS, CSTS_RDY};
use crate::error::NvmeError;
use crate::line::{Line, LINE_BYTES};
use crate::lo_hi::{join_lo_hi, split_lo_hi};
use crate::nvm::{NamespaceGeometry, Refusal};
use crate::rank::{is_cache_module, try_order};

fn ns_list(ids: &[u32]) -> [u8; 4096] {
    let mut page = [0u8; 4096];
    for (i, id) in ids.iter().enumerate() {
        page[i * 4..i * 4 + 4].copy_from_slice(&id.to_le_bytes());
    }
    page
}

#[test]
fn the_first_active_namespace_is_served_not_nsid_1() {
    assert_eq!(first_active_nsid(&ns_list(&[1, 2]), 2), Some(1));
    assert_eq!(first_active_nsid(&ns_list(&[2]), 4), Some(2), "NSID 1 deleted, 2 active");
    assert_eq!(first_active_nsid(&ns_list(&[]), 4), None, "no active namespace");
    // The broadcast id and one past NN are no namespace; the next entry is.
    assert_eq!(first_active_nsid(&ns_list(&[0xffff_ffff, 3]), 4), Some(3));
    assert_eq!(first_active_nsid(&ns_list(&[9, 3]), 4), Some(3));
    // NN of zero bounds nothing (the caller never asks then).
    assert_eq!(first_active_nsid(&ns_list(&[9]), 0), Some(9));
    // A zero ends the list, whatever follows it.
    assert_eq!(first_active_nsid(&ns_list(&[0, 5]), 8), None);
    // A full page of 1024 ids, the last the only valid one.
    let mut ids = vec![0xffff_ffffu32; 1023];
    ids.push(7);
    assert_eq!(first_active_nsid(&ns_list(&ids), 8), Some(7));
    // Any page: never panics, never answers 0, broadcast or past NN.
    let mut s = 0x1234_5678_9abc_def1u64;
    for _ in 0..20_000 {
        let mut page = [0u8; 4096];
        for b in page.iter_mut().take(64) {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            *b = s as u8;
        }
        let nn = (s >> 40) as u32 % 6;
        if let Some(id) = first_active_nsid(&page, nn) {
            assert!(id != 0 && id != 0xffff_ffff && (nn == 0 || id <= nn));
        }
    }
    // The page is read in whole entries even when cut short.
    assert_eq!(first_active_nsid(&[2, 0, 0], 4), None);
}

#[test]
fn only_a_controller_from_nvme_1_1_is_asked_for_the_list() {
    assert!(!lists_active_namespaces(0x0001_0000), "1.0 has no CNS 02h");
    assert!(lists_active_namespaces(0x0001_0100));
    assert!(lists_active_namespaces(0x0001_0400));
    assert!(lists_active_namespaces(0x0002_0000));
    assert!(lists_active_namespaces(0), "a blank VS is asked and falls back");
    assert_eq!(FALLBACK_NSID, 1);
}

#[test]
fn the_reset_waits_out_an_enable_firmware_left_half_done() {
    // CC.EN set, RDY and CFS clear: mid-enable, wait before clearing EN.
    assert_eq!(disable_start(CC_EN, 0), DisableStart::AwaitReady);
    // RDY already up, or a fatal controller: clear at once.
    assert_eq!(disable_start(CC_EN, CSTS_RDY), DisableStart::Clear);
    assert_eq!(disable_start(CC_EN, CSTS_CFS), DisableStart::Clear);
    // Disabled already (a RDY still set is finishing a disable): clear.
    assert_eq!(disable_start(0, 0), DisableStart::Clear);
    assert_eq!(disable_start(0, CSTS_RDY), DisableStart::Clear);
    // Gone from the bus, whatever CC read.
    assert_eq!(disable_start(CC_EN, u32::MAX), DisableStart::Gone);
    assert_eq!(disable_start(u32::MAX, u32::MAX), DisableStart::Gone);

    let cap = 20u64 << 24; // CAP.TO 20: 10 s
    assert!(matches!(pre_disable_step(cap, 0, 0), Ok(PreDisableStep::Wait)));
    assert!(matches!(pre_disable_step(cap, 0, 9_999), Ok(PreDisableStep::Wait)));
    assert!(matches!(pre_disable_step(cap, 0, 10_000), Ok(PreDisableStep::Clear)), "reset anyway");
    assert!(matches!(pre_disable_step(cap, CSTS_RDY, 3), Ok(PreDisableStep::Clear)));
    assert!(matches!(pre_disable_step(cap, CSTS_CFS, 3), Ok(PreDisableStep::Clear)));
    assert!(matches!(pre_disable_step(cap, u32::MAX, 0), Err(NvmeError::UnsupportedController)));
}

/// Run the pre-disable wait against CSTS `csts(ms)` on a clock that moves
/// `tick` each read and fails from read `fail_at` on.
fn pre_disable(
    cap: u64,
    tick: u64,
    fail_at: u64,
    csts: impl Fn(u64) -> u32,
) -> (Result<(), NvmeError>, u64) {
    let now = Cell::new(0u64);
    let reads = Cell::new(0u64);
    let r = await_ready_before_disable(
        cap,
        || csts(now.get()),
        || {
            reads.set(reads.get() + 1);
            assert!(reads.get() < 10_000_000, "the wait did not end");
            if reads.get() >= fail_at {
                return None;
            }
            let t = now.get();
            now.set(t + tick);
            Some(t)
        },
    );
    (r, now.get())
}

#[test]
fn the_pre_disable_wait_is_bounded_and_ends_on_rdy_or_cfs() {
    let cap = 20u64 << 24;
    let (r, at) = pre_disable(cap, 1, u64::MAX, |ms| if ms >= 700 { CSTS_RDY } else { 0 });
    assert!(r.is_ok() && (700..=702).contains(&at));
    let (r, at) = pre_disable(cap, 1, u64::MAX, |ms| if ms >= 50 { CSTS_CFS } else { 0 });
    assert!(r.is_ok() && at <= 52);
    // RDY never comes: the reset goes ahead once CAP.TO has passed.
    let (r, at) = pre_disable(cap, 3, u64::MAX, |_| 0);
    assert!(r.is_ok() && (10_000..=10_006).contains(&at), "{at}");
    let (r, _) = pre_disable(cap, 1, u64::MAX, |_| u32::MAX);
    assert!(matches!(r, Err(NvmeError::UnsupportedController)));
    // A clock that fails ends the wait, at the first read or midway.
    for fail_at in [1u64, 2, 500] {
        let (r, _) = pre_disable(cap, 1, fail_at, |_| 0);
        assert!(matches!(r, Err(NvmeError::ClockFailed)), "clock lost at read {fail_at}");
    }
}

#[test]
fn a_clock_that_fails_ends_the_ready_wait_instead_of_spinning() {
    for want in [true, false] {
        for fail_at in [1u64, 2, 3, 1000] {
            let reads = Cell::new(0u64);
            let r = wait_ready(
                20u64 << 24,
                want,
                || if want { 0 } else { CSTS_RDY },
                || {
                    reads.set(reads.get() + 1);
                    assert!(reads.get() < 10_000_000, "the wait did not end");
                    if reads.get() >= fail_at {
                        None
                    } else {
                        Some(reads.get())
                    }
                },
            );
            assert!(matches!(r, Err(NvmeError::ClockFailed)), "RDY={want}, lost at {fail_at}");
            assert_eq!(reads.get(), fail_at);
        }
    }
}

#[test]
fn a_budget_on_a_failed_clock_is_spent() {
    let b = Budget::begin(Some(1_000), 5_000);
    assert!(!b.spent(Some(1_000)));
    assert!(!b.spent(Some(5_999)));
    assert!(b.spent(Some(6_000)));
    assert!(!b.spent(Some(10)), "a clock that went back is no time passed");
    assert!(b.spent(None), "a failed read mid wait");
    let started_blind = Budget::begin(None, 5_000);
    for now in [None, Some(0), Some(1), Some(u64::MAX)] {
        assert!(started_blind.spent(now), "a wait begun on a failed read");
    }
    assert!(!Budget::begin(Some(u64::MAX), 1).spent(Some(u64::MAX)));
    assert!(Budget::begin(Some(0), 0).spent(Some(0)));
}

#[test]
fn a_64_bit_register_moves_low_dword_first_and_round_trips() {
    assert_eq!(split_lo_hi(0x0123_4567_89ab_cdef), (0x89ab_cdef, 0x0123_4567));
    assert_eq!(join_lo_hi(0x89ab_cdef, 0x0123_4567), 0x0123_4567_89ab_cdef);
    let mut s = 0xdead_beef_cafe_f00du64;
    for _ in 0..100_000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let (lo, hi) = split_lo_hi(s);
        assert_eq!(join_lo_hi(lo, hi), s);
        assert_eq!(lo as u64, s & 0xffff_ffff);
    }
    // An ASQ in the low 4 GiB writes a zero high dword.
    assert_eq!(split_lo_hi(0x7fff_f000), (0x7fff_f000, 0));
}

#[test]
fn an_optane_cache_module_is_tried_after_every_disk() {
    assert!(is_cache_module(0x8086, 0x2522));
    assert!(!is_cache_module(0x8086, 0x2700), "an Optane SSD is a disk");
    assert!(!is_cache_module(0x144d, 0x2522), "the id is Intel's alone");
    let ids = [(0x8086, 0x2522), (0x144d, 0xa808), (0x8086, 0xf1a8)];
    let mut order = [0usize; 4];
    assert_eq!(try_order(&ids, &mut order), 3);
    assert_eq!(&order[..3], &[1, 2, 0]);
    // Only a cache module: it is still tried.
    assert_eq!(try_order(&[(0x8086, 0x2522)], &mut order), 1);
    assert_eq!(order[0], 0);
    // More controllers than room: the first that fit, disks first.
    let many = [(1, 1), (0x8086, 0x2522), (2, 2), (3, 3), (4, 4), (5, 5)];
    assert_eq!(try_order(&many, &mut order), 4);
    assert_eq!(order, [0, 2, 3, 4]);
}

#[test]
fn the_served_controller_is_a_disk_with_io_and_a_failed_disk_is_retried() {
    use Outcome::{Failed, Io, NoIo};
    // (cache, outcome) in the order tried -> choice
    let cases: &[(&[(bool, Outcome)], Choice)] = &[
        (&[(false, Io)], Choice::Serve(0)),
        (&[(false, NoIo), (false, Io)], Choice::Serve(1)),
        (&[(false, Io), (true, Io)], Choice::Serve(0)),
        (&[(false, NoIo), (true, Io)], Choice::Serve(1)),
        (&[(false, Failed), (true, Io)], Choice::Retry),
        (&[(false, Failed), (false, Io)], Choice::Serve(1)),
        (&[(false, NoIo)], Choice::Serve(0)),
        (&[(false, NoIo), (true, NoIo)], Choice::Serve(0)),
        (&[(true, NoIo)], Choice::Serve(0)),
        (&[(true, Io)], Choice::Serve(0)),
        (&[(false, Failed)], Choice::Retry),
        (&[(true, Failed)], Choice::Retry),
        (&[], Choice::Retry),
    ];
    for (tries, want) in cases {
        assert_eq!(choose(tries), *want, "{tries:?}");
    }
    assert!(settled(false, Io));
    assert!(!settled(true, Io) && !settled(false, NoIo) && !settled(false, Failed));
    // Whatever the tries, a choice serves one that came up.
    let all = [Failed, NoIo, Io];
    for a in 0..6 {
        for b in 0..6 {
            for c in 0..6 {
                let tries = [(a >= 3, all[a % 3]), (b >= 3, all[b % 3]), (c >= 3, all[c % 3])];
                if let Choice::Serve(i) = choose(&tries) {
                    assert_ne!(tries[i].1, Failed);
                }
            }
        }
    }
}

#[test]
fn a_failed_command_is_named_by_its_status_fields() {
    let c = |status: u16| Completion { dw0: 0, dw1: 0, sq_head: 0, sq_id: 0, cid: 1, status };
    // Invalid Field in Command (generic, 02h), phase set, DNR set.
    let e = c((0x02 << 1) | 1 | 0x8000);
    assert_eq!((e.status_code_type(), e.status_code(), e.do_not_retry()), (0, 0x02, true));
    assert!(!e.successful());
    // Invalid Queue Identifier (command specific, 01h).
    let e = c((1 << 9) | (0x01 << 1));
    assert_eq!((e.status_code_type(), e.status_code(), e.do_not_retry()), (1, 0x01, false));
    // Unrecovered Read Error (media, 81h).
    let e = c((2 << 9) | (0x81 << 1));
    assert_eq!((e.status_code_type(), e.status_code()), (2, 0x81));
    assert_eq!(c(1).status_code(), 0);
    assert!(c(1).successful());
}

#[test]
fn an_unread_health_log_is_all_zero() {
    let parsed = SmartHealth::parse(&[0u8; 512]);
    let unread = SmartHealth::unread();
    assert_eq!(unread.critical_warning, parsed.critical_warning);
    assert_eq!(unread.temperature_kelvin, 0);
    assert_eq!(unread.data_units_written, parsed.data_units_written);
    assert_eq!(unread.media_errors, 0);
    assert_eq!(unread.critical_temp_time, parsed.critical_temp_time);
}

#[test]
fn a_refused_namespace_says_which_field_refused_it() {
    let ctrl = |mdts: u8| {
        let mut p = [0u8; 4096];
        p[0x4d] = mdts;
        ControllerIdentity::parse(&p)
    };
    let ns = |nsze: u64, nlbaf: u8, flbas: u8, ms: u16, lbads: u8| {
        let mut p = [0u8; 4096];
        p[0..8].copy_from_slice(&nsze.to_le_bytes());
        p[0x19] = nlbaf;
        p[0x1a] = flbas;
        let slot = 0x80 + (flbas as usize & 0xf) * 4;
        p[slot..slot + 2].copy_from_slice(&ms.to_le_bytes());
        p[slot + 2] = lbads;
        NamespaceIdentity::parse(1, &p)
    };
    let check = |n: &NamespaceIdentity| NamespaceGeometry::check(&ctrl(0), n).err();
    assert_eq!(
        NamespaceGeometry::check(&ctrl(0), &NamespaceIdentity::absent()).err(),
        Some(Refusal::NoNamespace)
    );
    assert_eq!(check(&ns(0, 0, 0, 0, 12)), Some(Refusal::Empty));
    assert_eq!(check(&ns(8, 0, 0, 0, 10)), Some(Refusal::BlockSize(1024)));
    assert_eq!(check(&ns(8, 0, 0, 8, 12)), Some(Refusal::Metadata(8)));
    assert_eq!(check(&ns(8, 1, 2, 0, 12)), Some(Refusal::Format { index: 2, upper: 0, nlbaf: 1 }));
    assert_eq!(
        check(&ns(8, 3, 0x21, 0, 12)),
        Some(Refusal::Format { index: 1, upper: 1, nlbaf: 3 })
    );
    assert_eq!(check(&ns(8, 0, 0, 0, 12)), None);
    assert_eq!(check(&ns(8, 0, 0, 0, 9)), None);
}

#[test]
fn a_console_line_reads_as_written_and_never_overruns() {
    let mut l = Line::new();
    l.text(b"controller ")
        .hex_digits(0x8086, 4)
        .text(b":")
        .hex_digits(0x2522, 4)
        .text(b" sc ")
        .hex(0x02, 2)
        .text(b" n ")
        .dec(0)
        .text(b" ")
        .dec(u64::MAX);
    assert_eq!(
        l.finish(),
        b"nvme: controller 8086:2522 sc 0x02 n 0 18446744073709551615\n".as_slice()
    );
    let mut long = Line::new();
    for _ in 0..100 {
        long.text(b"0123456789").hex(u64::MAX, 16).dec(12345);
    }
    let out = long.finish();
    assert_eq!(out.len(), LINE_BYTES);
    assert_eq!(*out.last().unwrap(), b'\n');
    let mut wide = Line::new();
    assert_eq!(wide.hex(0xabc, 40).finish(), b"nvme: 0x0000000000000abc\n".as_slice());
}
