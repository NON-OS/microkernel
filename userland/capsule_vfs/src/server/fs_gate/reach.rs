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

//! Whether a request may reach the paths it names: the guard's decision,
//! pure so the host proofs walk every case.

use alloc::string::String;

use super::guard::{guarded, touches};
use super::paths::named;
use crate::protocol::{OP_COPY, OP_RMDIR};

/// Whether a request `op` with payload `rest` (past the caller pid) is
/// served. Every path it names is normalized as its handler will; a copy or
/// removal, which can act on a whole subtree, is also held off the root and
/// any tree above a guarded one. `foreign` says whether the caller holds
/// ForeignExec, and is asked only when a guarded path is named.
pub fn reaches(
    op: u16,
    rest: &[u8],
    normalize: impl Fn(&str) -> Option<String>,
    foreign: impl FnOnce() -> bool,
) -> bool {
    let whole = matches!(op, OP_COPY | OP_RMDIR);
    let near = |raw: &&[u8]| {
        let path = core::str::from_utf8(raw).ok().and_then(&normalize);
        path.is_some_and(|p| if whole { touches(&p) } else { guarded(&p) })
    };
    !named(op, rest).iter().flatten().any(near) || foreign()
}
