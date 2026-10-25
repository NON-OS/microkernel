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

//! A `pkg` request put to the installer, with the two-step consent an install takes.

use super::super::call;
use super::{PkgDone, PkgOp};

/// Ask the installer. Two-step consent: the bare install only verifies the
/// package, and nothing is written until the person repeats it with --yes.
/// The commit carries the digest from the query, so a package swapped in
/// between the two steps is refused rather than installed.
pub(in super::super) fn perform(op: &PkgOp) -> PkgDone {
    match op {
        PkgOp::Install { path, yes } => match call::pkg_query(path) {
            Err(status) => PkgDone::QueryFailed(status),
            Ok(s) if !yes => PkgDone::Queried(s),
            Ok(s) => {
                let commit = call::pkg_commit(path, &s.digest);
                PkgDone::Committed(s, commit)
            }
        },
        PkgOp::Remove { name } => PkgDone::Removed(name.clone(), call::pkg_remove(name)),
    }
}
