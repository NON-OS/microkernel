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

//! The span list's ceiling, and random arguments.

use super::super::super::random::Regs;
use super::mmap::{FIXED, MAYMOVE, NOREPLACE};
use crate::linux::call::mem::map_args::{flags::MIN_ADDR, hint, map_args};
use crate::linux::call::mem::prot_args::{prot_span, wx_refused};
use crate::linux::call::mem::remap_args::{end_of, remap_args, unmap_span};
use crate::linux::guest::{maps_full, page_len, MAX_MAPS, PAGE, USER_MAX};

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
