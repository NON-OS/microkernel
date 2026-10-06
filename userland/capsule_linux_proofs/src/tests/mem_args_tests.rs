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

//! mmap, mprotect, mremap and munmap refuse from their arguments alone what
//! Linux refuses, with Linux's errno, before any mapping is looked at; and
//! whatever span one accepts lies inside the guest's area without wrapping.

use super::random::Regs;
use crate::linux::abi::errno::{EINVAL, ENOMEM, EPERM};
use crate::linux::call::mem::map_args::{hint, map_args, MIN_ADDR};
use crate::linux::call::mem::prot_args::{prot_span, wx_refused};
use crate::linux::call::mem::remap_args::{end_of, remap_args, unmap_span};
use crate::linux::guest::{maps_full, page_len, MAX_MAPS, PAGE, USER_MAX};

const PRIVATE: u64 = 0x02;
const SHARED: u64 = 0x01;
const VALIDATE: u64 = 0x03;
const ANON: u64 = 0x20;
const FIXED: u64 = 0x10;
const NOREPLACE: u64 = 0x10_0000;
const RW: u64 = 3;
const RX: u64 = 5;
const RWX: u64 = 7;
const GROWSDOWN: u64 = 0x0100_0000;
const GROWSUP: u64 = 0x0200_0000;
const MAYMOVE: u64 = 1;
const MFIXED: u64 = 2;
const DONTUNMAP: u64 = 4;

/// addr, len, prot, flags, off, and what mmap's rule answers.
type MapRow = (u64, u64, u64, u64, u64, Result<(), i64>);
/// addr, len, prot, and what mprotect's rule answers.
type ProtRow = (u64, u64, u64, Result<Option<(u64, u64)>, i64>);

#[test]
fn mmap_refuses_each_bad_argument_with_linux_errno() {
    let table: [MapRow; 16] = [
        (0, 0, RW, PRIVATE | ANON, 0, Err(EINVAL)),
        (0, PAGE, RW, PRIVATE | ANON, 1, Err(EINVAL)),
        (0, PAGE, RW, PRIVATE | ANON, PAGE + 8, Err(EINVAL)),
        // Neither shared nor private, and a type Linux does not define.
        (0, PAGE, RW, ANON, 0, Err(EINVAL)),
        (0, PAGE, RW, 0x0f | ANON, 0, Err(EINVAL)),
        (0x10_0001, PAGE, RW, PRIVATE | ANON | FIXED, 0, Err(EINVAL)),
        (0x10_0001, PAGE, RW, PRIVATE | ANON | NOREPLACE, 0, Err(EINVAL)),
        (0, u64::MAX, RW, PRIVATE | ANON, 0, Err(ENOMEM)),
        (0, PAGE, RWX, PRIVATE | ANON, 0, Err(EPERM)),
        (0, PAGE, 6, PRIVATE | ANON, 0, Err(EPERM)),
        (0, PAGE, RW, PRIVATE | ANON | FIXED, 0, Err(EPERM)),
        (PAGE, PAGE, RW, PRIVATE | ANON | FIXED, 0, Err(EPERM)),
        (MIN_ADDR - PAGE, PAGE, RW, PRIVATE | ANON | NOREPLACE, 0, Err(EPERM)),
        (MIN_ADDR, PAGE, RW, PRIVATE | ANON | FIXED, 0, Ok(())),
        (0, PAGE, RX, SHARED | ANON, 0, Ok(())),
        (0, 1, 0, VALIDATE, PAGE, Ok(())),
    ];
    for (addr, len, prot, flags, off, want) in table {
        assert_eq!(map_args(addr, len, prot, flags, off), want, "{addr:#x} {len:#x} {flags:#x}");
    }
}

#[test]
fn mmap_checks_its_arguments_before_its_own_policy() {
    // A misaligned offset is EINVAL even when the protection is refused too.
    assert_eq!(map_args(0, PAGE, RWX, PRIVATE | ANON, 3), Err(EINVAL));
    // A hint, unlike an exact address, may be anywhere at all.
    assert_eq!(map_args(1, PAGE, RW, PRIVATE | ANON, 0), Ok(()));
}

#[test]
fn a_hint_is_rounded_down_and_never_below_mmap_min_addr() {
    assert_eq!(hint(0), None);
    assert_eq!(hint(PAGE - 1), None, "inside page zero is no hint at all");
    assert_eq!(hint(PAGE), Some(MIN_ADDR));
    assert_eq!(hint(MIN_ADDR - 1), Some(MIN_ADDR));
    assert_eq!(hint(0x1234_5678), Some(0x1234_5000));
    assert_eq!(hint(u64::MAX), Some(!(PAGE - 1)));
}

#[test]
fn mprotect_refuses_each_bad_argument_with_linux_errno() {
    let at = 0x10_0000;
    let table: [ProtRow; 14] = [
        (at + 1, PAGE, 1, Err(EINVAL)),
        (at, PAGE, GROWSDOWN | GROWSUP | 1, Err(EINVAL)),
        // An empty span succeeds before its protection is looked at.
        (at, 0, 0xff, Ok(None)),
        // An end that wraps is ENOMEM, as Linux's `end <= start` has it.
        (at, u64::MAX, 1, Err(ENOMEM)),
        (at, u64::MAX - at, 1, Err(ENOMEM)),
        (at, PAGE, 0x10, Err(EINVAL)),
        (at, PAGE, 1 << 40, Err(EINVAL)),
        (at, PAGE, GROWSDOWN | 1, Err(EINVAL)),
        (at, PAGE, GROWSUP, Err(EINVAL)),
        (at, PAGE, RWX, Err(EPERM)),
        (USER_MAX, PAGE, 1, Err(ENOMEM)),
        (USER_MAX - PAGE, PAGE + 1, 1, Err(ENOMEM)),
        // PROT_SEM is accepted, as x86 accepts it.
        (at, 1, 8 | 1, Ok(Some((at, PAGE)))),
        (USER_MAX - PAGE, PAGE, RX, Ok(Some((USER_MAX - PAGE, PAGE)))),
    ];
    for (addr, len, prot, want) in table {
        assert_eq!(prot_span(addr, len, prot), want, "{addr:#x} {len:#x} {prot:#x}");
    }
}

#[test]
fn mremap_refuses_each_bad_argument_with_einval() {
    let at = 0x10_0000;
    // (old, old_len, new_len, flags)
    let refused: [(u64, u64, u64, u64); 11] = [
        (at, PAGE, 2 * PAGE, 8),
        (at, PAGE, 2 * PAGE, MFIXED),
        (at, PAGE, 2 * PAGE, MFIXED | MAYMOVE),
        (at, PAGE, PAGE, DONTUNMAP),
        (at, PAGE, 2 * PAGE, DONTUNMAP | MAYMOVE),
        (at, PAGE, PAGE, DONTUNMAP | MAYMOVE),
        (at + 8, PAGE, 2 * PAGE, MAYMOVE),
        (at, PAGE, 0, MAYMOVE),
        (at, 0, PAGE, MAYMOVE),
        // Lengths that round past the top: PAGE_ALIGN wraps them to zero.
        (at, PAGE, u64::MAX, MAYMOVE),
        (at, u64::MAX - 1, PAGE, MAYMOVE),
    ];
    for (old, old_len, new_len, flags) in refused {
        assert_eq!(remap_args(old, old_len, new_len, flags), Err(EINVAL), "{old_len:#x} {flags}");
    }
    assert_eq!(remap_args(at, 1, PAGE + 1, MAYMOVE), Ok((PAGE, 2 * PAGE)));
    assert_eq!(remap_args(at, 3 * PAGE, PAGE, 0), Ok((3 * PAGE, PAGE)));
}

#[test]
fn a_span_end_never_wraps_or_passes_the_guest_area() {
    assert_eq!(end_of(USER_MAX - PAGE, PAGE), Some(USER_MAX));
    assert_eq!(end_of(USER_MAX, 1), None);
    assert_eq!(end_of(1 << 63, 1 << 63), None);
    assert_eq!(end_of(MIN_ADDR, u64::MAX - MIN_ADDR + PAGE), None, "wraps to a low address");
}

#[test]
fn munmap_refuses_each_bad_argument_with_einval() {
    let at = 0x10_0000;
    for (addr, len) in
        [(at + 1, PAGE), (at, 0), (USER_MAX, 1), (at, u64::MAX), (u64::MAX - PAGE + 1, PAGE)]
    {
        assert_eq!(unmap_span(addr, len), Err(EINVAL), "{addr:#x} {len:#x}");
    }
    assert_eq!(unmap_span(at, 1), Ok((at, PAGE)));
    assert_eq!(unmap_span(USER_MAX - PAGE, PAGE), Ok((USER_MAX - PAGE, PAGE)));
}

#[test]
fn the_span_list_stops_at_linux_max_map_count_but_may_always_shrink() {
    assert_eq!(MAX_MAPS, 65_530, "vm.max_map_count's default");
    assert!(!maps_full(MAX_MAPS - 1, MAX_MAPS));
    assert!(maps_full(MAX_MAPS, MAX_MAPS + 1), "one more mapping is ENOMEM");
    assert!(maps_full(MAX_MAPS - 1, MAX_MAPS + 1), "a split past the ceiling is ENOMEM");
    // A guest already past it (a ceiling lowered under it) can give spans back.
    assert!(!maps_full(MAX_MAPS + 9, MAX_MAPS + 8));
    assert!(!maps_full(MAX_MAPS + 9, MAX_MAPS + 9));
}

/// Random registers into every rule: none panics, and whatever is accepted
/// is a page-aligned span that neither wraps nor leaves the guest's area.
#[test]
fn random_arguments_never_panic_and_accepted_spans_stay_inside() {
    let mut r = Regs::new(0x6d65_6d5f_6172_6773);
    for _ in 0..200_000 {
        let (addr, len, prot, flags, off) = (r.arg(), r.arg(), r.small(1 << 4), r.arg(), r.arg());
        if map_args(addr, len, prot, flags, off).is_ok() {
            assert!(len > 0 && off % PAGE == 0 && page_len(len).is_some() && !wx_refused(prot));
            assert!(matches!(flags & 0x0f, 1..=3));
            if flags & (FIXED | NOREPLACE) != 0 {
                assert!(addr % PAGE == 0 && addr >= MIN_ADDR);
            }
        }
        if let Some(at) = hint(addr) {
            assert!(at % PAGE == 0 && at >= MIN_ADDR && at <= addr.max(MIN_ADDR));
        }
        let prot = if r.small(2) == 0 { r.arg() } else { prot };
        if let Ok(Some((start, span))) = prot_span(addr, len, prot) {
            assert!(start == addr && span % PAGE == 0 && span >= len && span > 0);
            assert!(start.checked_add(span).is_some_and(|e| e <= USER_MAX));
            assert!(!wx_refused(prot));
        }
        let (new_len, rflags) = (r.arg(), r.small(8));
        if let Ok((o, n)) = remap_args(addr, len, new_len, rflags) {
            assert!(o % PAGE == 0 && n % PAGE == 0 && o >= len && n >= new_len && o > 0 && n > 0);
            assert_eq!(rflags & !MAYMOVE, 0, "only MAYMOVE is ever offered");
        }
        if let Ok((start, span)) = unmap_span(addr, len) {
            assert!(span > 0 && span % PAGE == 0 && start + span <= USER_MAX);
        }
        if let Some(end) = end_of(addr, len) {
            assert!(end >= addr && end - addr == len && end <= USER_MAX);
        }
    }
}
