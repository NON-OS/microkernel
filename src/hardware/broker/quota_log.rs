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

//! A grant refused for its holder's quota, named on the console. The
//! map answers ENOMEM either way; this line tells a spent quota from a
//! spent window. Rate-limited, as every diag site is.

use super::quota::Spent;

static QUOTA: crate::sys::diag::Site = crate::sys::diag::Site::new(b"broker.quota");

/// `kind` is "MMIO" or "DMA".
pub(super) fn refused(pid: u32, kind: &str, spent: Spent) {
    let why = match spent {
        Spent::Grants => "grant count quota spent",
        Spent::Bytes => "byte quota spent",
    };
    QUOTA.reject(kind, why, pid);
}
