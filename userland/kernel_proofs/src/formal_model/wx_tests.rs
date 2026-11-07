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


//! W^X preservation (`Formal.wx_preservation`). Every map path refuses a
//! permission `is_wx_violation` flags, and the model's `Perm.wx` is that
//! test; a permission that passes is encoded by `to_pte_flags` as a leaf
//! that is not writable and executable at once. The surface view
//! permissions (`map_frames`) are the model's never-executable attach.

use crate::memory::paging::constants::{PERM_EXECUTE, PERM_WRITE, PTE_NO_EXECUTE, PTE_WRITABLE};
use crate::memory::paging::types::permissions::PagePermissions;

use super::protection::ProtectionFlags;
use super::pte_flags as user_pte;

fn leaf_is_wx(flags: u64) -> bool {
    flags & PTE_WRITABLE != 0 && flags & PTE_NO_EXECUTE == 0
}

#[test]
fn every_permission_the_map_gate_passes_installs_a_non_wx_leaf() {
    for bits in 0u32..(1 << 16) {
        let p = PagePermissions::from_bits(bits);
        let model_wx = bits & PERM_WRITE != 0 && bits & PERM_EXECUTE != 0;
        assert_eq!(p.is_wx_violation(), model_wx, "{bits:#x}");
        if !p.is_wx_violation() {
            assert!(!leaf_is_wx(p.to_pte_flags()), "{bits:#x}");
        }
    }
}

#[test]
fn surface_views_are_never_executable() {
    for p in [PagePermissions::user_rw(), PagePermissions::user_ro()] {
        assert!(!p.is_wx_violation());
        assert_ne!(p.to_pte_flags() & PTE_NO_EXECUTE, 0);
    }
}

fn every_protection() -> impl Iterator<Item = ProtectionFlags> {
    (0u8..16).map(|b| ProtectionFlags::new(b & 1 != 0, b & 2 != 0, b & 4 != 0, b & 8 != 0))
}

/// `AddressSpace::map_page` refuses exactly the W+X requests, which is the
/// model's `Perm.wx`; write-only and execute-only requests pass.
#[test]
fn address_space_map_refuses_exactly_write_and_execute() {
    for p in every_protection() {
        assert_eq!(p.is_wx_violation(), p.write && p.execute, "{p:?}");
    }
    assert!(!ProtectionFlags::new(true, true, false, true).is_wx_violation());
    assert!(!ProtectionFlags::new(true, false, true, true).is_wx_violation());
    assert!(ProtectionFlags::new(true, true, true, true).is_wx_violation());
}

/// Every request that passes encodes a leaf that is not writable and
/// executable at once, and the default request passes.
#[test]
fn an_accepted_address_space_request_never_encodes_a_wx_leaf() {
    for p in every_protection().filter(|p| !p.is_wx_violation()) {
        let f = p.to_pte_flags();
        assert!(!(f & user_pte::WRITABLE != 0 && f & user_pte::NO_EXECUTE == 0), "{p:?}");
    }
    assert!(!ProtectionFlags::default().is_wx_violation());
}

/// The leaf bits the address-space encoder writes are distinct single bits,
/// and the address mask holds none of them, so a permission bit never
/// aliases another or the frame address.
#[test]
fn address_space_leaf_bits_are_distinct() {
    let bits = [
        user_pte::PRESENT,
        user_pte::WRITABLE,
        user_pte::USER_ACCESSIBLE,
        user_pte::WRITE_THROUGH,
        user_pte::NO_CACHE,
        user_pte::ACCESSED,
        user_pte::DIRTY,
        user_pte::HUGE_PAGE,
        user_pte::GLOBAL,
        user_pte::NO_EXECUTE,
    ];
    let mut seen = 0u64;
    for b in bits {
        assert_eq!(b.count_ones(), 1);
        assert_eq!(seen & b, 0);
        assert_eq!(user_pte::ADDR_MASK & b, 0);
        seen |= b;
    }
}
