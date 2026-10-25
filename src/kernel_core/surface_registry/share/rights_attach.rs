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

//! Who attaches a surface, pure; kernel_proofs::surface_closing::
//! rights. The owner and the receivers it named; anyone else is refused the
//! same way whether or not the handle names a surface, so trying handles
//! learns nothing.

use super::rights_rule::Right;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Attach {
    Owner,
    Receiver { write: bool },
    Refused,
}

/// `owner`: whose surface `handle` names now, None when it names none.
/// `same`: whether two pids are threads of one process.
pub(crate) fn attach(
    rights: &[Right],
    owner: Option<u32>,
    handle: u64,
    pid: u32,
    same: impl Fn(u32, u32) -> bool,
) -> Attach {
    let Some(owner) = owner else { return Attach::Refused };
    if same(owner, pid) {
        return Attach::Owner;
    }
    match rights.iter().find(|r| r.handle == handle && same(r.pid, pid)) {
        Some(r) => Attach::Receiver { write: r.write },
        None => Attach::Refused,
    }
}
