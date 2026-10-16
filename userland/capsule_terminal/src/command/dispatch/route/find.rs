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
//! Look a shell name up in every group's table.

use super::files::{Files, NAMES as FILES};
use super::info::{Info, NAMES as INFO};
use super::net::{Net, NAMES as NET};
use super::session::{Session, NAMES as SESSION};

/// The one name that hands the rest of the line to nox.
const NOX: &[u8] = b"nox";

/// Which group a name belongs to, and which command within it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Shell {
    Nox,
    Info(Info),
    Session(Session),
    Files(Files),
    Net(Net),
}

fn look<T: Copy>(table: &[(&[u8], T)], name: &[u8]) -> Option<T> {
    table.iter().find(|(n, _)| *n == name).map(|&(_, cmd)| cmd)
}

/// The command a name runs, or None for a name the shell passes to nox.
pub(crate) fn route(name: &[u8]) -> Option<Shell> {
    if name == NOX {
        return Some(Shell::Nox);
    }
    look(INFO, name)
        .map(Shell::Info)
        .or_else(|| look(SESSION, name).map(Shell::Session))
        .or_else(|| look(FILES, name).map(Shell::Files))
        .or_else(|| look(NET, name).map(Shell::Net))
}

/// Every name and alias the shell answers to itself, group by group.
/// Only host tests that mount this module call it; the shell itself routes.
#[cfg(test)]
pub(crate) fn names() -> impl Iterator<Item = &'static [u8]> {
    let info = INFO.iter().map(|&(n, _)| n);
    let session = SESSION.iter().map(|&(n, _)| n);
    let files = FILES.iter().map(|&(n, _)| n);
    let net = NET.iter().map(|&(n, _)| n);
    core::iter::once(NOX).chain(info).chain(session).chain(files).chain(net)
}
