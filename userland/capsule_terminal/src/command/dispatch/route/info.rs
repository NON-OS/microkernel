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
//! The shell names that report on the system and this terminal.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Info {
    About,
    Bench,
    Version,
    Whoami,
    Receipt,
    Audit,
    Log,
    Capsules,
    Display,
    Neofetch,
    Motd,
    Market,
}

pub(crate) const NAMES: &[(&[u8], Info)] = &[
    (b"about", Info::About),
    (b"bench", Info::Bench),
    (b"version", Info::Version),
    (b"whoami", Info::Whoami),
    (b"receipt", Info::Receipt),
    (b"audit", Info::Audit),
    (b"log", Info::Log),
    (b"capsules", Info::Capsules),
    (b"caps", Info::Capsules),
    (b"display", Info::Display),
    (b"neofetch", Info::Neofetch),
    (b"motd", Info::Motd),
    (b"market", Info::Market),
];
