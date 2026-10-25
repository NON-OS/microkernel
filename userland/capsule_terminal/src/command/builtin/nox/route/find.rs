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
//! Look a nox name up in every group's table.

use super::apps::{Apps, NAMES as APPS};
use super::files::{Files, NAMES as FILES};
use super::host::{Host, NAMES as HOST};
use super::net::{Net, NAMES as NET};
use super::shell::{Shell, NAMES as SHELL};

/// Which group a name belongs to, and which command within it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Nox {
    Files(Files),
    Net(Net),
    Apps(Apps),
    Shell(Shell),
    Host(Host),
}

fn look<T: Copy>(table: &[(&[u8], T)], name: &[u8]) -> Option<T> {
    table.iter().find(|(n, _)| *n == name).map(|&(_, cmd)| cmd)
}

/// The command a name runs, or None for a name nox does not know.
pub(crate) fn route(name: &[u8]) -> Option<Nox> {
    look(FILES, name)
        .map(Nox::Files)
        .or_else(|| look(NET, name).map(Nox::Net))
        .or_else(|| look(APPS, name).map(Nox::Apps))
        .or_else(|| look(SHELL, name).map(Nox::Shell))
        .or_else(|| look(HOST, name).map(Nox::Host))
}

/// Every name and alias nox answers to, group by group.
/// Only host tests that mount this module call it; the shell itself routes.
#[cfg(test)]
pub(crate) fn names() -> impl Iterator<Item = &'static [u8]> {
    let files = FILES.iter().map(|&(n, _)| n);
    let net = NET.iter().map(|&(n, _)| n);
    let apps = APPS.iter().map(|&(n, _)| n);
    let shell = SHELL.iter().map(|&(n, _)| n);
    let host = HOST.iter().map(|&(n, _)| n);
    files.chain(net).chain(apps).chain(shell).chain(host)
}
