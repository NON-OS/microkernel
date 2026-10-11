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

//! The map's cap column is the cap table's: each call passes with its cap
//! alone and with Admin, and with every other cap but not those two it is
//! refused. Every driver-grant call is in the map, and nothing else is.

extern crate alloc;
use alloc::format;
use alloc::vec::Vec;

use super::table::SURFACE;
use super::token::token;
use crate::capabilities::Capability;
use crate::syscall::abi::REGISTRY;
use crate::syscall::contract::is_allowed;
use crate::syscall::numbers::SyscallNumber;

#[test]
fn each_call_needs_exactly_its_cap_or_admin() {
    for row in SURFACE {
        let (call, cap) = (row.0, row.1);
        assert!(is_allowed(&token(alloc::vec![cap]), call), "{call:?} with {cap:?}");
        assert!(is_allowed(&token(alloc::vec![Capability::Admin]), call), "{call:?}, Admin");
        let rest: Vec<Capability> = Capability::all()
            .iter()
            .copied()
            .filter(|c| *c != cap && *c != Capability::Admin)
            .collect();
        assert!(!is_allowed(&token(rest), call), "{call:?} without {cap:?}");
    }
}

const PREFIXES: [&str; 7] =
    ["MkDma", "MkMmio", "MkIrq", "MkPio", "MkPciConfig", "MkDeviceList", "MkDeviceClaim"];

fn is_driver_grant(n: SyscallNumber) -> bool {
    let name = format!("{n:?}");
    name == "MkDeviceRelease" || PREFIXES.iter().any(|p| name.starts_with(p))
}

#[test]
fn the_map_holds_every_driver_grant_call_and_nothing_else() {
    // Every registered call, from the ABI registry itself: driver calls are
    // four-letter tags, far above any small range a scan would cover.
    let calls: Vec<SyscallNumber> = REGISTRY
        .iter()
        .flat_map(|d| d.iter().map(|e| e.variant))
        .filter(|&n| is_driver_grant(n))
        .collect();
    for n in &calls {
        assert!(SURFACE.iter().any(|r| r.0 == *n), "{n:?} is not in the map");
    }
    for r in SURFACE {
        assert!(calls.contains(&r.0), "{:?} is not a registered driver-grant call", r.0);
    }
    assert_eq!(calls.len(), SURFACE.len());
}
