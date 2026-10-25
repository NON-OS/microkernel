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
//! The nox names that change or show the shell itself.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Shell {
    Env,
    Set,
    Unset,
    Alias,
    Unalias,
    Echo,
    History,
    Display,
    Clear,
    Help,
    Motd,
}

pub(crate) const NAMES: &[(&[u8], Shell)] = &[
    (b"env", Shell::Env),
    (b"set", Shell::Set),
    (b"unset", Shell::Unset),
    (b"alias", Shell::Alias),
    (b"unalias", Shell::Unalias),
    (b"echo", Shell::Echo),
    (b"history", Shell::History),
    (b"display", Shell::Display),
    (b"clear", Shell::Clear),
    (b"help", Shell::Help),
    (b"motd", Shell::Motd),
];
