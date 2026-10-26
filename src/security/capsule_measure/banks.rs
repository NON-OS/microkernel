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

//! The banks the kernel's events carry: every bank the firmware's log
//! declares, in a fixed order, so the kernel's events parse in the same log;
//! SHA-256 alone when the loader carried no log. `None` when the log declares
//! a bank this kernel cannot hash, for then it can extend no event the log
//! would accept.

use alloc::vec::Vec;

use super::algs::ALG_SHA256;
use super::declared::from_declared;
use crate::boot::handoff::types::MODULE_KIND_TCG_LOG;
use crate::security::boot::modules::module_bytes;

const LOOK: u64 = 4 * 1024 * 1024;

pub(super) fn banks() -> Option<Vec<u16>> {
    let Some(log) = module_bytes(MODULE_KIND_TCG_LOG, LOOK) else {
        return Some(alloc::vec![ALG_SHA256]);
    };
    let (declared, _) = nonos_boot_measure::tcg::spec_id(log).ok()?;
    from_declared(declared.len(), |alg| declared.size_of(alg))
}
