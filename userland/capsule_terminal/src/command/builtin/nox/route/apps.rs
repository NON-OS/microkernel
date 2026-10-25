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
//! The nox names that find, install and start apps and packages.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Apps {
    Market,
    Run,
    Exec,
    Install,
    Pkg,
    Git,
}

pub(crate) const NAMES: &[(&[u8], Apps)] = &[
    (b"apps", Apps::Market),
    (b"market", Apps::Market),
    (b"run", Apps::Run),
    (b"open", Apps::Run),
    (b"exec", Apps::Exec),
    (b"install", Apps::Install),
    (b"pkg", Apps::Pkg),
    (b"git", Apps::Git),
];
