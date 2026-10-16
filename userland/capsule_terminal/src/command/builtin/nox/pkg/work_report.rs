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

//! What `pkg` says once the installer has answered.

use alloc::vec::Vec;

use super::super::emit;
use super::super::summary::slug;
use super::PkgDone;
use crate::command::output::Output;

/// Say what the installer answered. True when it went through.
pub(in super::super) fn report(out: &mut Output<'_>, done: &PkgDone) -> bool {
    match done {
        PkgDone::QueryFailed(status) => {
            emit::error(out, *status);
            false
        }
        PkgDone::Queried(s) => {
            emit::summary(out, s);
            out.writeln(b"run again with --yes to install");
            true
        }
        PkgDone::Committed(s, commit) => {
            emit::summary(out, s);
            match commit {
                Ok(()) => {
                    out.writeln(&said(b"installed ", slug(&s.namespace)));
                    true
                }
                Err(status) => {
                    emit::error(out, *status);
                    false
                }
            }
        }
        PkgDone::Removed(name, Ok(())) => {
            out.writeln(&said(b"removed ", name));
            true
        }
        PkgDone::Removed(_, Err(status)) => {
            emit::error(out, *status);
            false
        }
    }
}

fn said(what: &[u8], name: &[u8]) -> Vec<u8> {
    let mut line = Vec::with_capacity(what.len() + name.len());
    line.extend_from_slice(what);
    line.extend_from_slice(name);
    line
}
