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
//! The nox names that ask about this machine and its processes.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Host {
    Date,
    Kill,
    Uptime,
    Battery,
    Caps,
    Ps,
    Svc,
    Id,
    Sys,
}

pub(crate) const NAMES: &[(&[u8], Host)] = &[
    (b"date", Host::Date),
    (b"kill", Host::Kill),
    (b"uptime", Host::Uptime),
    (b"battery", Host::Battery),
    (b"bat", Host::Battery),
    (b"caps", Host::Caps),
    (b"ps", Host::Ps),
    (b"svc", Host::Svc),
    (b"id", Host::Id),
    (b"sys", Host::Sys),
];
